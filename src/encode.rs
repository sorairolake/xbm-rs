// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Encodes XBM images.

use std::{io, io::Write};

/// Encoder for XBM images.
#[derive(Debug)]
pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    /// Creates a new `Encoder`.
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Encodes the binary image `buf`.
    ///
    /// `0` represents a white pixel and `1` represents a black pixel.
    ///
    /// `width` should be a multiple of 8.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if an error occurs during I/O operations.
    ///
    /// # Panics
    ///
    /// Panics if any of the following are true:
    ///
    /// - The length of `buf` and the image dimensions (the width multiplied by
    ///   the height) are different.
    /// - `buf` contains values other than `0` and `1`.
    /// - Only one of `x_hot` and `y_hot` is [`Some`].
    pub fn encode(
        mut self,
        buf: &[u8],
        name: &str,
        width: u32,
        height: u32,
        x_hot: Option<u32>,
        y_hot: Option<u32>,
    ) -> Result<(), Error> {
        let width = usize::try_from(width).expect("width should be in the range of `usize`");
        let dimensions = usize::try_from(height).map(|h| width * h);
        assert_eq!(
            Ok(buf.len()),
            dimensions,
            "`buf` and the image dimensions are different"
        );
        assert!(
            !buf.iter().any(|&p| p > 1),
            "`buf` contains values other than `0` and `1`"
        );
        assert_eq!(
            x_hot.is_some(),
            y_hot.is_some(),
            "only one of `x_hot` and `y_hot` is `Some`"
        );

        writeln!(self.writer, "#define {name}_width {width}")?;
        writeln!(self.writer, "#define {name}_height {height}")?;
        if let Some(pos) = x_hot {
            writeln!(self.writer, "#define {name}_x_hot {pos}")?;
        }
        if let Some(pos) = y_hot {
            writeln!(self.writer, "#define {name}_y_hot {pos}")?;
        }

        writeln!(self.writer, "static unsigned char {name}_bits[] = {{")?;
        let mut pixels_chunk = Vec::with_capacity(12);
        for per_line in buf.chunks(width) {
            for chunk in per_line.chunks(8) {
                let mut pixels = u8::default();
                for (i, pixel) in chunk.iter().enumerate() {
                    pixels |= pixel << i;
                }
                pixels_chunk.push(pixels);
                if pixels_chunk.len() == 12 {
                    let line = pixels_chunk
                        .iter()
                        .map(|p| format!("{p:#04X}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    writeln!(self.writer, "    {line},")?;
                    pixels_chunk.clear();
                }
            }
        }
        if !pixels_chunk.is_empty() {
            let line = pixels_chunk
                .into_iter()
                .map(|p| format!("{p:#04X}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(self.writer, "    {line},")?;
        }
        writeln!(self.writer, "}};")
    }
}

#[cfg(feature = "image")]
impl<W: Write> image::ImageEncoder for Encoder<W> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        use image::{
            error::{EncodingError, ImageFormatHint},
            ExtendedColorType, ImageError,
        };

        match color_type {
            ExtendedColorType::L1 => self
                .encode(buf, "image", width, height, None, None)
                .map_err(ImageError::IoError),
            ExtendedColorType::L8 => {
                let mut buf = buf.to_vec();
                buf.iter_mut().for_each(|p| *p = u8::from(*p < 128));
                self.encode(&buf, "image", width, height, None, None)
                    .map_err(ImageError::IoError)
            }
            _ => Err(ImageError::Encoding(EncodingError::new(
                ImageFormatHint::Name(String::from("XBM")),
                format!("unsupported color type `{color_type:?}`"),
            ))),
        }
    }
}

/// The error type indicating that an error occurred during encoding.
pub type Error = io::Error;

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    #[test]
    fn encode() {
        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [u8::default(); 132];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder.encode(pixels, "image", 8, 7, None, None).unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/basic.xbm")
        );
    }

    #[test]
    fn encode_16x14() {
        // "B" (16x14)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
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

        let mut buf = [u8::default(); 268];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder.encode(pixels, "image", 16, 14, None, None).unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/16x14.xbm")
        );
    }

    #[test]
    fn encode_width_7() {
        // "I" (7x6)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\
\x00\x00\x00\x01\x00\x00\x00\
\x00\x00\x00\x01\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\
\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [u8::default(); 126];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder.encode(pixels, "image", 7, 6, None, None).unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/width_7.xbm")
        );
    }

    #[test]
    fn encode_width_14() {
        // "I" (14x12)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
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

        let mut buf = [u8::default(); 240];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder.encode(pixels, "image", 14, 12, None, None).unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/width_14.xbm")
        );
    }

    #[test]
    fn encode_with_hotspot() {
        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [u8::default(); 176];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder
            .encode(pixels, "image", 8, 7, Some(4), Some(3))
            .unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/hotspot.xbm")
        );
    }

    #[test]
    #[should_panic(expected = "`buf` and the image dimensions are different")]
    fn encode_with_invalid_dimensions() {
        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [];
        let encoder = Encoder::new(buf.as_mut_slice());
        let _: Result<(), Error> = encoder.encode(pixels, "image", 4, 3, None, None);
    }

    #[test]
    #[should_panic(expected = "`buf` contains values other than `0` and `1`")]
    fn encode_from_invalid_pixels() {
        // "B" (8x7)
        let pixels = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";

        let mut buf = [];
        let encoder = Encoder::new(buf.as_mut_slice());
        let _: Result<(), Error> = encoder.encode(pixels, "image", 8, 7, None, None);
    }

    #[test]
    #[should_panic(expected = "only one of `x_hot` and `y_hot` is `Some`")]
    fn encode_with_only_x_hot_some() {
        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [];
        let encoder = Encoder::new(buf.as_mut_slice());
        let _: Result<(), Error> = encoder.encode(pixels, "image", 8, 7, Some(4), None);
    }

    #[test]
    #[should_panic(expected = "only one of `x_hot` and `y_hot` is `Some`")]
    fn encode_with_only_y_hot_some() {
        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [];
        let encoder = Encoder::new(buf.as_mut_slice());
        let _: Result<(), Error> = encoder.encode(pixels, "image", 8, 7, None, Some(3));
    }

    #[cfg(feature = "image")]
    #[test]
    fn image_encoder_from_l1() {
        use image::{ExtendedColorType, ImageEncoder};

        // "B" (8x7)
        let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x01\x00\x00\x01\x00\x00\
\x00\x00\x01\x01\x01\x00\x00\x00\
\x00\x00\x00\x00\x00\x00\x00\x00";

        let mut buf = [u8::default(); 132];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder
            .write_image(pixels, 8, 7, ExtendedColorType::L1)
            .unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/basic.xbm")
        );
    }

    #[cfg(feature = "image")]
    #[test]
    fn image_encoder_from_l8() {
        use image::{ExtendedColorType, ImageEncoder};

        // "B" (8x7)
        let pixels = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\x00\xFF\xFF\x00\xFF\xFF\
\xFF\xFF\x00\x00\x00\xFF\xFF\xFF\
\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";

        let mut buf = [u8::default(); 132];
        let encoder = Encoder::new(buf.as_mut_slice());
        encoder
            .write_image(pixels, 8, 7, ExtendedColorType::L8)
            .unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/basic.xbm")
        );
    }

    #[cfg(feature = "image")]
    #[test]
    fn image_encoder_from_unsupported_color_type() {
        use image::{ExtendedColorType, ImageEncoder};

        // A black pixel (1x1)
        let pixels = [u8::MIN; 3];

        let mut buf = [];
        let encoder = Encoder::new(buf.as_mut_slice());
        let result = encoder.write_image(&pixels, 1, 1, ExtendedColorType::Rgb8);
        assert!(result.is_err());
    }

    #[cfg(feature = "image")]
    #[test]
    fn png_to_xbm() {
        let input = image::open("tests/data/qr_code.png").unwrap();
        let (width, height) = (input.width(), input.height());
        let mut pixels = input.into_bytes();
        pixels.iter_mut().for_each(|p| *p = u8::from(*p < 128));

        let mut buf = Vec::with_capacity(69460);
        let encoder = Encoder::new(buf.by_ref());
        encoder
            .encode(&pixels, "qr_code", width, height, None, None)
            .unwrap();
        assert_eq!(
            str::from_utf8(&buf).unwrap(),
            include_str!("../tests/data/qr_code.xbm")
        );
    }
}
