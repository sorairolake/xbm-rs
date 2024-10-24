// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Encodes XBM images.

use std::io::{self, Write};

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
        self,
        buf: impl AsRef<[u8]>,
        name: impl AsRef<str>,
        width: u32,
        height: u32,
        x_hot: Option<u32>,
        y_hot: Option<u32>,
    ) -> Result<(), Error> {
        let inner = |mut encoder: Self,
                     buf: &[u8],
                     name: &str,
                     width: u32,
                     height: u32,
                     x_hot: Option<u32>,
                     y_hot: Option<u32>|
         -> Result<(), Error> {
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

            writeln!(encoder.writer, "#define {name}_width {width}")?;
            writeln!(encoder.writer, "#define {name}_height {height}")?;
            if let Some(pos) = x_hot {
                writeln!(encoder.writer, "#define {name}_x_hot {pos}")?;
            }
            if let Some(pos) = y_hot {
                writeln!(encoder.writer, "#define {name}_y_hot {pos}")?;
            }

            writeln!(encoder.writer, "static unsigned char {name}_bits[] = {{")?;
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
                        writeln!(encoder.writer, "    {line},")?;
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
                writeln!(encoder.writer, "    {line},")?;
            }
            writeln!(encoder.writer, "}};")
        };
        inner(
            self,
            buf.as_ref(),
            name.as_ref(),
            width,
            height,
            x_hot,
            y_hot,
        )
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
                self.encode(buf, "image", width, height, None, None)
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
    use super::*;

    #[test]
    fn error_type() {
        assert_eq!(
            std::any::type_name::<Error>(),
            std::any::type_name::<io::Error>()
        );
    }
}
