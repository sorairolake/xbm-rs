// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Decodes XBM images.

use std::{
    error, fmt,
    io::{self, BufRead, Seek, SeekFrom},
    num::ParseIntError,
};

/// Decoder for XBM images.
#[derive(Debug)]
pub struct Decoder<R: BufRead + Seek> {
    reader: R,
    name: String,
    width: u32,
    height: u32,
    x_hot: Option<u32>,
    y_hot: Option<u32>,
}

impl<R: BufRead + Seek> Decoder<R> {
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// assert!(Decoder::new(reader).is_ok());
    /// ```
    pub fn new(mut reader: R) -> Result<Self, Error> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let mut tokens = buf.split_whitespace();
        if tokens.next() != Some("#define") {
            return Err(Error::InvalidHeader);
        }
        let Some(name) = tokens
            .next()
            .filter(|t| t.ends_with("_width"))
            .map(|t| t.trim_end_matches("_width"))
            .filter(|n| {
                let mut chars = n.chars();
                chars.next().is_some_and(unicode_ident::is_xid_start)
                    && chars.all(unicode_ident::is_xid_continue)
            })
        else {
            return Err(Error::InvalidHeader);
        };
        let Some(width) = tokens.next().map(str::parse).transpose()? else {
            return Err(Error::InvalidHeader);
        };
        if tokens.next().is_some() {
            return Err(Error::InvalidHeader);
        }

        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let mut tokens = buf.split_whitespace();
        if tokens.next() != Some("#define") || tokens.next() != Some(&format!("{name}_height")) {
            return Err(Error::InvalidHeader);
        }
        let Some(height) = tokens.next().map(str::parse).transpose()? else {
            return Err(Error::InvalidHeader);
        };
        if tokens.next().is_some() {
            return Err(Error::InvalidHeader);
        }

        let pos = reader.stream_position()?;

        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let mut tokens = buf.split_whitespace();
        let x_hot = if tokens.next() == Some("#define") {
            if tokens.next() != Some(&format!("{name}_x_hot")) {
                return Err(Error::InvalidHeader);
            }
            let Some(value) = tokens.next().map(str::parse).transpose()? else {
                return Err(Error::InvalidHeader);
            };
            if tokens.next().is_some() {
                return Err(Error::InvalidHeader);
            }
            Some(value)
        } else {
            reader.seek(SeekFrom::Start(pos))?;
            Option::default()
        };

        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let mut tokens = buf.split_whitespace();
        let y_hot = if tokens.next() == Some("#define") {
            if tokens.next() != Some(&format!("{name}_y_hot")) {
                return Err(Error::InvalidHeader);
            }
            let Some(value) = tokens.next().map(str::parse).transpose()? else {
                return Err(Error::InvalidHeader);
            };
            if tokens.next().is_some() {
                return Err(Error::InvalidHeader);
            }
            Some(value)
        } else {
            reader.seek(SeekFrom::Start(pos))?;
            Option::default()
        };

        if x_hot.is_some() != y_hot.is_some() {
            return Err(Error::InvalidHeader);
        }

        let pos = reader.stream_position()?;
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        if buf.starts_with(&format!("static unsigned char {name}_bits[] = {{"))
            || buf.starts_with(&format!("static char {name}_bits[] = {{"))
        {
            let Some(index) = buf
                .find('{')
                .and_then(|i| i.checked_add(1))
                .map(u64::try_from)
                .transpose()
                .ok()
                .flatten()
            else {
                return Err(Error::InvalidHeader);
            };
            reader.seek(SeekFrom::Start(pos + index))?;
        } else {
            return Err(Error::InvalidHeader);
        }
        let name = name.into();
        Ok(Self {
            reader,
            name,
            width,
            height,
            x_hot,
            y_hot,
        })
    }

    /// Returns the name of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.name(), "image");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the width of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.width(), 8);
    /// ```
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.height(), 7);
    /// ```
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the _x_ coordinate of the hotspot.
    ///
    /// Returns [`None`] if the value is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.x_hot(), None);
    ///
    /// let reader = File::open("tests/data/hotspot.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.x_hot(), Some(4));
    /// ```
    #[inline]
    pub const fn x_hot(&self) -> Option<u32> {
        self.x_hot
    }

    /// Returns the _y_ coordinate of the hotspot.
    ///
    /// Returns [`None`] if the value is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.y_hot(), None);
    ///
    /// let reader = File::open("tests/data/hotspot.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    /// assert_eq!(decoder.y_hot(), Some(3));
    /// ```
    #[inline]
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// // "B" (8x7)
    /// let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x01\x00\x00\x01\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x01\x00\x00\x01\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x00\x00\x00\x00\x00\x00";
    ///
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    ///
    /// let mut buf = [u8::default(); 56];
    /// decoder.decode(&mut buf).unwrap();
    /// assert_eq!(buf, *expected);
    /// ```
    pub fn decode(self, buf: &mut (impl AsMut<[u8]> + ?Sized)) -> Result<(), Error> {
        let inner = |decoder: Self, buf: &mut [u8]| -> Result<(), Error> {
            let buf_len = buf.len();
            let width =
                usize::try_from(decoder.width()).expect("width should be in the range of `usize`");
            let dimensions = usize::try_from(decoder.height()).map(|h| width * h);
            assert_eq!(
                Ok(buf_len),
                dimensions,
                "`buf` and the image dimensions are different"
            );

            let mut pixels = [u8::default(); 8];
            let mut remaining_pixels = width;
            let mut pos = usize::default();

            let mut lines_iter = decoder.reader.lines().peekable();
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

                    if !pixels_hex.is_ascii()
                        || pixels_hex.len() != 4
                        || !pixels_hex.starts_with("0x")
                    {
                        return Err(Error::InvalidHexByte(pixels_hex));
                    }
                    let pixels_hex = pixels_hex.trim_start_matches("0x");
                    let pixels_byte = u8::from_str_radix(pixels_hex, 16)?;

                    for (i, pixel) in pixels.iter_mut().enumerate() {
                        *pixel = (pixels_byte >> i) & 1;
                    }

                    if remaining_pixels < 8 {
                        buf[pos..(pos + remaining_pixels)]
                            .copy_from_slice(&pixels[..remaining_pixels]);
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
        };
        inner(self, buf.as_mut())
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{fs::File, io::BufReader};
    /// #
    /// # use xbm::Decoder;
    /// #
    /// // "B" (8x7)
    /// let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x01\x00\x00\x01\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x01\x00\x00\x01\x00\x00\
    ///                  \x00\x00\x01\x01\x01\x00\x00\x00\
    ///                  \x00\x00\x00\x00\x00\x00\x00\x00";
    ///
    /// let reader = File::open("tests/data/basic.xbm")
    ///     .map(BufReader::new)
    ///     .unwrap();
    /// let decoder = Decoder::new(reader).unwrap();
    ///
    /// let buf = decoder.decode_to_vec().unwrap();
    /// assert_eq!(buf, expected);
    /// ```
    #[inline]
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
    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    #[inline]
    fn color_type(&self) -> image::ColorType {
        use image::ColorType;

        ColorType::L8
    }

    fn read_image(self, buf: &mut [u8]) -> image::ImageResult<()> {
        use image::{
            ImageError,
            error::{DecodingError, ImageFormatHint},
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

    #[inline]
    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> image::ImageResult<()> {
        (*self).read_image(buf)
    }

    #[inline]
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
    #[inline]
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseIntError> for Error {
    #[inline]
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error as _, io::ErrorKind, str::FromStr};

    use super::*;

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
            format!("{:?}", Error::Io(io::Error::from(ErrorKind::NotFound))),
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
            format!("{}", Error::Io(io::Error::from(ErrorKind::NotFound))),
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
        assert!(
            Error::Io(io::Error::from(ErrorKind::NotFound))
                .source()
                .unwrap()
                .is::<io::Error>()
        );
        assert!(
            Error::ParseInt(u32::from_str("").unwrap_err())
                .source()
                .unwrap()
                .is::<ParseIntError>()
        );
    }

    #[test]
    fn from_io_error_to_error() {
        let err = io::Error::from(ErrorKind::NotFound);
        assert!(matches!(Error::from(err), Error::Io(_)));
    }

    #[test]
    fn from_parse_int_error_to_error() {
        let err = u32::from_str("").unwrap_err();
        assert!(matches!(Error::from(err), Error::ParseInt(_)));
    }
}
