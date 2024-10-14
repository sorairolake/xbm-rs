// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Decodes XBM images.

use std::{
    error, fmt, io,
    io::{BufRead, Seek, SeekFrom},
    num::ParseIntError,
};

/// Decoder for XBM images.
#[derive(Debug)]
pub struct Decoder<R: BufRead + Seek> {
    reader: R,
    width: u32,
    height: u32,
    x_hot: Option<u32>,
    y_hot: Option<u32>,
}

impl<R: BufRead + Seek> Decoder<R> {
    #[allow(clippy::missing_panics_doc)]
    /// Creates a new `Decoder`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any of the following are true:
    ///
    /// - The header is invalid.
    /// - An error occurs during I/O operations.
    /// - An error occurs while parsing either the width, the height, or the
    ///   hotspot.
    pub fn new(mut reader: R) -> Result<Self, Error> {
        let mut buf = String::new();

        reader.read_line(&mut buf)?;
        let tokens: Vec<_> = buf.split_whitespace().collect();
        let cond = tokens.len() == 3 && tokens[0] == "#define" && tokens[1].ends_with("_width");
        if !cond {
            return Err(Error::InvalidHeader);
        }
        let width = tokens[2].parse()?;
        buf.clear();

        reader.read_line(&mut buf)?;
        let tokens: Vec<_> = buf.split_whitespace().collect();
        let cond = tokens.len() == 3 && tokens[0] == "#define" && tokens[1].ends_with("_height");
        if !cond {
            return Err(Error::InvalidHeader);
        }
        let height = tokens[2].parse()?;
        buf.clear();

        let pos = reader.stream_position()?;
        let (mut x_hot, mut y_hot) = (Option::default(), Option::default());

        reader.read_line(&mut buf)?;
        if buf.starts_with("#define") {
            let tokens: Vec<_> = buf.split_whitespace().collect();
            let cond = tokens.len() == 3 && tokens[0] == "#define" && tokens[1].ends_with("_x_hot");
            if !cond {
                return Err(Error::InvalidHeader);
            }
            x_hot = tokens[2].parse().map(Some)?;
        } else {
            reader.seek(SeekFrom::Start(pos))?;
        }
        buf.clear();

        reader.read_line(&mut buf)?;
        if buf.starts_with("#define") {
            let tokens: Vec<_> = buf.split_whitespace().collect();
            let cond = tokens.len() == 3 && tokens[0] == "#define" && tokens[1].ends_with("_y_hot");
            if !cond {
                return Err(Error::InvalidHeader);
            }
            y_hot = tokens[2].parse().map(Some)?;
        } else {
            reader.seek(SeekFrom::Start(pos))?;
        }
        if x_hot.is_some() != y_hot.is_some() {
            return Err(Error::InvalidHeader);
        }
        buf.clear();

        let pos = reader.stream_position()?;
        reader.read_line(&mut buf)?;
        let cond = (buf.starts_with("static unsigned char") || buf.starts_with("static char"))
            && buf.matches("_bits[] = {").count() == 1
            && buf.matches('{').count() == 1;
        if cond {
            let index = u64::try_from(buf.find('{').expect("`{` must be found") + 1)
                .expect("the index should be in the range of `u64`");
            reader.seek(SeekFrom::Start(pos + index))?;
        } else {
            return Err(Error::InvalidHeader);
        }
        Ok(Self {
            reader,
            width,
            height,
            x_hot,
            y_hot,
        })
    }

    /// Returns the width of the image.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the _x_ coordinate of the hotspot.
    ///
    /// Returns [`None`] if the value is not defined.
    pub const fn x_hot(&self) -> Option<u32> {
        self.x_hot
    }

    /// Returns the _y_ coordinate of the hotspot.
    ///
    /// Returns [`None`] if the value is not defined.
    pub const fn y_hot(&self) -> Option<u32> {
        self.y_hot
    }

    /// Decodes the image into `buf`.
    ///
    /// `0` represents a white pixel and `1` represents a black pixel.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any of the following are true:
    ///
    /// - The hex byte value is invalid.
    /// - The image termination string is not `};`.
    /// - The expected image dimensions and the actual image dimensions
    ///   mismatch.
    /// - An error occurs during I/O operations.
    /// - An error occurs while parsing the hex byte value.
    ///
    /// # Panics
    ///
    /// Panics if the length of `buf` and the image dimensions (the width
    /// multiplied by the height) are different.
    pub fn decode(self, buf: &mut [u8]) -> Result<(), Error> {
        let buf_len = buf.len();
        let width = usize::try_from(self.width()).expect("width should be in the range of `usize`");
        let dimensions = usize::try_from(self.height()).map(|h| width * h);
        assert_eq!(
            Ok(buf_len),
            dimensions,
            "`buf` and the image dimensions are different"
        );

        let mut pixels = [u8::default(); 8];
        let mut remaining_pixels = width;
        let mut pos = usize::default();

        let mut lines_iter = self.reader.lines().peekable();
        while let Some(line) = lines_iter.next() {
            let line = line?;
            let mut line = line.trim();

            if lines_iter.peek().is_none() {
                if !line.ends_with("};") {
                    return Err(Error::InvalidTermination);
                }
                line = line.trim_end_matches("};");
                if line.is_empty() {
                    break;
                }
            }

            let mut line_iter = line
                .split_terminator(',')
                .map(str::trim)
                .map(String::from)
                .peekable();
            while let Some(pixels_hex) = line_iter.next() {
                if line_iter.peek().is_none() && pixels_hex.is_empty() {
                    break;
                }

                if !pixels_hex.is_ascii() || pixels_hex.len() != 4 || !pixels_hex.starts_with("0x")
                {
                    return Err(Error::InvalidHexByte(pixels_hex));
                }
                let pixels_hex = pixels_hex.trim_start_matches("0x");
                let pixels_byte = u8::from_str_radix(pixels_hex, 16)?;

                for (i, pixel) in pixels.iter_mut().enumerate() {
                    *pixel = (pixels_byte >> i) & 1;
                }

                if remaining_pixels < 8 {
                    buf[pos..(pos + remaining_pixels)].copy_from_slice(&pixels[..remaining_pixels]);
                    pos += remaining_pixels;
                    remaining_pixels = width;
                } else {
                    buf[pos..(pos + 8)].copy_from_slice(&pixels);
                    pos += 8;
                    remaining_pixels -= 8;
                    if remaining_pixels == 0 {
                        remaining_pixels = width;
                    }
                }
            }
        }

        if pos == buf_len {
            Ok(())
        } else {
            Err(Error::InvalidImageSize(pos))
        }
    }

    #[allow(clippy::missing_panics_doc)]
    /// Decodes the image into a newly allocated [`Vec`].
    ///
    /// `0` represents a white pixel and `1` represents a black pixel.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any of the following are true:
    ///
    /// - The hex byte value is invalid.
    /// - The image termination string is not `};`.
    /// - The expected image dimensions and the actual image dimensions
    ///   mismatch.
    /// - An error occurs during I/O operations.
    /// - An error occurs while parsing the hex byte value.
    pub fn decode_to_vec(self) -> Result<Vec<u8>, Error> {
        let dimensions = usize::try_from(self.width())
            .expect("width should be in the range of `usize`")
            * usize::try_from(self.height()).expect("height should be in the range of `usize`");
        let mut buf = vec![u8::default(); dimensions];
        self.decode(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "image")]
impl<R: BufRead + Seek> image::ImageDecoder for Decoder<R> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn color_type(&self) -> image::ColorType {
        use image::ColorType;

        ColorType::L8
    }

    fn read_image(self, buf: &mut [u8]) -> image::ImageResult<()> {
        use image::{
            error::{DecodingError, ImageFormatHint},
            ImageError,
        };

        self.decode(buf).map_err(|err| match err {
            Error::Io(err) => ImageError::IoError(err),
            err => ImageError::Decoding(DecodingError::new(
                ImageFormatHint::Name(String::from("XBM")),
                err,
            )),
        })?;
        debug_assert!(!buf.iter().any(|&p| p > 1));
        buf.iter_mut()
            .for_each(|p| *p = if p == &0 { u8::MAX } else { u8::MIN });
        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> image::ImageResult<()> {
        (*self).read_image(buf)
    }

    fn original_color_type(&self) -> image::ExtendedColorType {
        use image::ExtendedColorType;

        ExtendedColorType::L1
    }
}

/// The error type indicating that an error occurred during decoding.
#[derive(Debug)]
pub enum Error {
    /// The header was invalid.
    InvalidHeader,

    /// The byte value expressed in the [C hexadecimal notation] which
    /// represents the pixels was invalid.
    ///
    /// [C hexadecimal notation]: https://en.wikipedia.org/wiki/Hexadecimal
    InvalidHexByte(String),

    /// The image termination string was not `};`.
    InvalidTermination,

    /// The expected image dimensions and the actual image dimensions
    /// mismatched.
    InvalidImageSize(usize),

    /// An error occurred during I/O operations.
    Io(io::Error),

    /// An error occurred while parsing an integer.
    ParseInt(ParseIntError),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHeader => write!(f, "invalid header"),
            Self::InvalidHexByte(value) => write!(f, "invalid hex byte `{value}`"),
            Self::InvalidTermination => write!(f, "invalid termination string"),
            Self::InvalidImageSize(size) => write!(f, "invalid image size `{size}`"),
            Self::Io(err) => err.fmt(f),
            Self::ParseInt(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::ParseInt(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error as _,
        fs::File,
        io::{BufReader, Cursor},
        num::IntErrorKind,
        str::FromStr,
    };

    use super::*;

    #[test]
    fn decode() {
        // "B" (8x7)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        {
            let reader = File::open("tests/data/basic.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 8);
            assert_eq!(decoder.height(), 7);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 56];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
        {
            let reader = File::open("tests/data/basic_minified.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 8);
            assert_eq!(decoder.height(), 7);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 56];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
    }

    #[test]
    fn decode_lower_hex() {
        // "B" (8x7)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let reader = File::open("tests/data/basic_lower_hex.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        assert_eq!(decoder.width(), 8);
        assert_eq!(decoder.height(), 7);
        assert_eq!(decoder.x_hot(), None);
        assert_eq!(decoder.y_hot(), None);
        let mut buf = [u8::default(); 56];
        decoder.decode(buf.as_mut_slice()).unwrap();
        assert_eq!(buf.as_slice(), expected);
    }

    #[test]
    fn decode_to_vec() {
        // "B" (8x7)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let reader = File::open("tests/data/basic.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        let buf = decoder.decode_to_vec().unwrap();
        assert_eq!(buf.len(), 56);
        assert_eq!(buf.as_slice(), expected);
    }

    #[test]
    fn decode_16x14() {
        // "B" (16x14)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

        {
            let reader = File::open("tests/data/16x14.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 16);
            assert_eq!(decoder.height(), 14);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 224];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
        {
            let reader = File::open("tests/data/16x14_minified.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 16);
            assert_eq!(decoder.height(), 14);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 224];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
    }

    #[test]
    fn decode_width_7() {
        // "I" (7x6)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\
\x00\x00\x00\x01\x00\x00\x00\
\x00\x00\x00\x01\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\
\x00\x00\x00\x00\x00\x00\x00";

        {
            let reader = File::open("tests/data/width_7.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 7);
            assert_eq!(decoder.height(), 6);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 42];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
        {
            let reader = File::open("tests/data/width_7_minified.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 7);
            assert_eq!(decoder.height(), 6);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 42];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
    }

    #[test]
    fn decode_width_14() {
        // "I" (14x12)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x01\x01\x01\x01\x01\x01\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

        {
            let reader = File::open("tests/data/width_14.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 14);
            assert_eq!(decoder.height(), 12);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 168];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
        {
            let reader = File::open("tests/data/width_14_minified.xbm")
                .map(BufReader::new)
                .unwrap();
            let decoder = Decoder::new(reader).unwrap();
            assert_eq!(decoder.width(), 14);
            assert_eq!(decoder.height(), 12);
            assert_eq!(decoder.x_hot(), None);
            assert_eq!(decoder.y_hot(), None);
            let mut buf = [u8::default(); 168];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }
    }

    #[test]
    fn decode_with_hotspot() {
        // "B" (8x7)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let reader = File::open("tests/data/hotspot.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        assert_eq!(decoder.width(), 8);
        assert_eq!(decoder.height(), 7);
        assert_eq!(decoder.x_hot(), Some(4));
        assert_eq!(decoder.y_hot(), Some(3));
        let mut buf = [u8::default(); 56];
        decoder.decode(buf.as_mut_slice()).unwrap();
        assert_eq!(buf.as_slice(), expected);
    }

    #[test]
    fn decode_without_unsigned() {
        // "B" (8x7)
        let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let reader = File::open("tests/data/without_unsigned.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        assert_eq!(decoder.width(), 8);
        assert_eq!(decoder.height(), 7);
        assert_eq!(decoder.x_hot(), None);
        assert_eq!(decoder.y_hot(), None);
        let mut buf = [u8::default(); 56];
        decoder.decode(buf.as_mut_slice()).unwrap();
        assert_eq!(buf.as_slice(), expected);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn decode_with_invalid_header() {
        {
            let image = "#include image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
        {
            let image = "#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }

        {
            let image = "#define image_width 8
#include image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
        {
            let image = "#define image_width 8
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }

        {
            let image = "#define image_width 8
#define image_height 7
#include image_x_hot 4
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }

        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4
#include image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }

        {
            let image = "#define image_width 8
#define image_height 7
static unsigned short image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
        {
            let image = "#define image_width 8
#define image_height 7
static short image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert!(matches!(err, Error::InvalidHeader));
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn decode_with_invalid_header_value() {
        {
            let image = "#define image_width 4294967296
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::PosOverflow
            );
        }
        {
            let image = "#define image_width -1
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }
        {
            let image = "#define image_width a
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }

        {
            let image = "#define image_width 8
#define image_height 4294967296
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::PosOverflow
            );
        }
        {
            let image = "#define image_width 8
#define image_height -1
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }
        {
            let image = "#define image_width 8
#define image_height a
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }

        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4294967296
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::PosOverflow
            );
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot -1
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot a
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }

        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4
#define image_y_hot 4294967296
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::PosOverflow
            );
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4
#define image_y_hot -1
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }
        {
            let image = "#define image_width 8
#define image_height 7
#define image_x_hot 4
#define image_y_hot a
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let buf = Cursor::new(image);
            let err = Decoder::new(buf).unwrap_err();
            assert_eq!(
                err.source()
                    .unwrap()
                    .downcast_ref::<ParseIntError>()
                    .unwrap()
                    .kind(),
                &IntErrorKind::InvalidDigit
            );
        }
    }

    #[test]
    fn decode_with_invalid_hex_byte() {
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    🦀, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            if let Error::InvalidHexByte(value) = err {
                assert_eq!(value, "🦀");
            } else {
                unreachable!();
            }
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 1c, 0x24, 0x1C, 0x00,
};
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            if let Error::InvalidHexByte(value) = err {
                assert_eq!(value, "1c");
            } else {
                unreachable!();
            }
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0b00,
};
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            if let Error::InvalidHexByte(value) = err {
                assert_eq!(value, "0b00");
            } else {
                unreachable!();
            }
        }
    }

    #[test]
    fn decode_with_invalid_termination() {
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C }; 0x24, 0x1C, 0x24, 0x1C, 0x00,
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C};0x24, 0x1C, 0x24, 0x1C, 0x00,
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, }; 0x24, 0x1C, 0x24, 0x1C, 0x00,
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C,};0x24, 0x1C, 0x24, 0x1C, 0x00,
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }

        {
            // "B" (8x7)
            let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
}; 
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            decoder.decode(buf.as_mut_slice()).unwrap();
            assert_eq!(buf.as_slice(), expected);
        }

        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }
        {
            let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00
";
            let image = Cursor::new(image);
            let decoder = Decoder::new(image).unwrap();
            let mut buf = [u8::default(); 56];
            let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
            assert!(matches!(err, Error::InvalidTermination));
        }
    }

    #[test]
    fn decode_with_invalid_image_size() {
        let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C,
};
";
        let image = Cursor::new(image);
        let decoder = Decoder::new(image).unwrap();
        let mut buf = [u8::default(); 56];
        let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
        if let Error::InvalidImageSize(size) = err {
            assert_eq!(size, 48);
        } else {
            unreachable!();
        }
    }

    #[test]
    #[should_panic(expected = "range end index 64 out of range for slice of length 56")]
    fn decode_from_too_large_image() {
        let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00, 0x00,
};
";
        let image = Cursor::new(image);
        let decoder = Decoder::new(image).unwrap();
        let mut buf = [u8::default(); 56];
        let _: Result<(), Error> = decoder.decode(buf.as_mut_slice());
    }

    #[test]
    fn decode_from_invalid_hex_byte_value() {
        let image = "#define image_width 8
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0xgg, 0x24, 0x1C, 0x00,
};
";
        let image = Cursor::new(image);
        let decoder = Decoder::new(image).unwrap();
        let mut buf = [u8::default(); 56];
        let err = decoder.decode(buf.as_mut_slice()).unwrap_err();
        assert_eq!(
            err.source()
                .unwrap()
                .downcast_ref::<ParseIntError>()
                .unwrap()
                .kind(),
            &IntErrorKind::InvalidDigit
        );
    }

    #[test]
    #[should_panic(expected = "`buf` and the image dimensions are different")]
    fn decode_with_invalid_buffer() {
        let reader = File::open("tests/data/basic.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        let mut buf = [];
        let _: Result<(), Error> = decoder.decode(buf.as_mut_slice());
    }

    #[cfg(feature = "image")]
    #[test]
    fn image_decoder() {
        use image::{ColorType, ExtendedColorType, ImageDecoder};

        // "B" (8x7)
        let expected = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";

        let reader = File::open("tests/data/basic.xbm")
            .map(BufReader::new)
            .unwrap();
        let mut decoder = Decoder::new(reader).unwrap();
        assert_eq!(decoder.dimensions(), (8, 7));
        assert_eq!(decoder.color_type(), ColorType::L8);
        assert_eq!(decoder.original_color_type(), ExtendedColorType::L1);
        assert_eq!(decoder.icc_profile().unwrap(), None);
        assert_eq!(decoder.total_bytes(), 56);
        let mut buf = [u8::default(); 56];
        decoder.read_image(buf.as_mut_slice()).unwrap();
        assert_eq!(buf.as_slice(), expected);
    }

    #[cfg(feature = "image")]
    #[test]
    fn xbm_to_png() {
        use image::{ColorType, ImageFormat};

        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        let (width, height) = (decoder.width(), decoder.height());
        assert_eq!(width, 296);
        assert_eq!(height, 296);
        assert_eq!(decoder.x_hot(), None);
        assert_eq!(decoder.y_hot(), None);
        let mut buf =
            vec![u8::default(); usize::try_from(width).unwrap() * usize::try_from(height).unwrap()];
        decoder.decode(buf.as_mut_slice()).unwrap();

        buf.iter_mut()
            .for_each(|p| *p = if p == &0 { u8::MAX } else { u8::MIN });
        let mut writer = Cursor::new(Vec::with_capacity(2091));
        image::write_buffer_with_format(
            &mut writer,
            &buf,
            width,
            height,
            ColorType::L8,
            ImageFormat::Png,
        )
        .unwrap();

        let actual = image::load_from_memory(writer.get_ref()).unwrap();
        let expected = image::open("tests/data/qr_code.png").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn debug_error() {
        assert_eq!(format!("{:?}", Error::InvalidHeader), "InvalidHeader");
        assert_eq!(
            format!("{:?}", Error::InvalidHexByte(String::from("0b00"))),
            r#"InvalidHexByte("0b00")"#
        );
        assert_eq!(
            format!("{:?}", Error::InvalidTermination),
            "InvalidTermination"
        );
        assert_eq!(
            format!("{:?}", Error::InvalidImageSize(usize::default())),
            "InvalidImageSize(0)"
        );
        assert_eq!(
            format!("{:?}", Error::Io(io::Error::from(io::ErrorKind::NotFound))),
            "Io(Kind(NotFound))"
        );
        assert_eq!(
            format!("{:?}", Error::ParseInt(u32::from_str("").unwrap_err())),
            "ParseInt(ParseIntError { kind: Empty })"
        );
    }

    #[test]
    fn display_error() {
        assert_eq!(format!("{}", Error::InvalidHeader), "invalid header");
        assert_eq!(
            format!("{}", Error::InvalidHexByte(String::from("0b00"))),
            "invalid hex byte `0b00`"
        );
        assert_eq!(
            format!("{}", Error::InvalidTermination),
            "invalid termination string"
        );
        assert_eq!(
            format!("{}", Error::InvalidImageSize(usize::default())),
            "invalid image size `0`"
        );
        assert_eq!(
            format!("{}", Error::Io(io::Error::from(io::ErrorKind::NotFound))),
            "entity not found"
        );
        assert_eq!(
            format!("{}", Error::ParseInt(u32::from_str("").unwrap_err())),
            "cannot parse integer from empty string"
        );
    }

    #[test]
    fn source_error() {
        assert!(Error::InvalidHeader.source().is_none());
        assert!(Error::InvalidHexByte(String::default()).source().is_none());
        assert!(Error::InvalidTermination.source().is_none());
        assert!(Error::InvalidImageSize(usize::default()).source().is_none());
        assert!(Error::Io(io::Error::from(io::ErrorKind::NotFound))
            .source()
            .unwrap()
            .is::<io::Error>());
        assert!(Error::ParseInt(u32::from_str("").unwrap_err())
            .source()
            .unwrap()
            .is::<ParseIntError>());
    }

    #[test]
    fn from_io_error_to_error() {
        let err = io::Error::from(io::ErrorKind::NotFound);
        assert!(matches!(Error::from(err), Error::Io(_)));
    }

    #[test]
    fn from_parse_int_error_to_error() {
        let err = u32::from_str("").unwrap_err();
        assert!(matches!(Error::from(err), Error::ParseInt(_)));
    }
}
