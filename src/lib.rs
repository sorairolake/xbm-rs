// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `xbm` crate is a [XBM] encoding and decoding library.
//!
//! This crate supports the [X version 11 bitmap file format].
//!
//! The width and the height of XBM are unlimited, but in this crate they are
//! limited to [`u32`].
//!
//! # Examples
//!
//! ## Encoding a XBM file
//!
//! ```
//! use std::io::Write;
//!
//! use xbm::Encoder;
//!
//! // "B" (8x7)
//! let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
//!                \x00\x00\x01\x01\x01\x00\x00\x00\
//!                \x00\x00\x01\x00\x00\x01\x00\x00\
//!                \x00\x00\x01\x01\x01\x00\x00\x00\
//!                \x00\x00\x01\x00\x00\x01\x00\x00\
//!                \x00\x00\x01\x01\x01\x00\x00\x00\
//!                \x00\x00\x00\x00\x00\x00\x00\x00";
//!
//! let mut buf = [u8::default(); 131];
//! let encoder = Encoder::new(buf.as_mut_slice());
//! encoder.encode(pixels, "image", 8, 7, None, None).unwrap();
//! assert_eq!(buf, *include_bytes!("../tests/data/basic.xbm"));
//! ```
//!
//! ### `image` crate support
//!
//! ```
//! # #[cfg(feature = "image")]
//! # {
//! use std::io::Write;
//!
//! use xbm::Encoder;
//!
//! let input = image::open("tests/data/qr_code.png").unwrap();
//!
//! let mut buf = Vec::with_capacity(69453);
//! let encoder = Encoder::new(buf.by_ref());
//! input.write_with_encoder(encoder).unwrap();
//! assert_eq!(buf, include_bytes!("../tests/data/qr_code.xbm"));
//! # }
//! ```
//!
//! ## Decoding a XBM file
//!
//! ```
//! use std::{fs::File, io::BufReader};
//!
//! use xbm::Decoder;
//!
//! // "B" (8x7)
//! let expected = b"\x00\x00\x00\x00\x00\x00\x00\x00\
//!                  \x00\x00\x01\x01\x01\x00\x00\x00\
//!                  \x00\x00\x01\x00\x00\x01\x00\x00\
//!                  \x00\x00\x01\x01\x01\x00\x00\x00\
//!                  \x00\x00\x01\x00\x00\x01\x00\x00\
//!                  \x00\x00\x01\x01\x01\x00\x00\x00\
//!                  \x00\x00\x00\x00\x00\x00\x00\x00";
//!
//! let reader = File::open("tests/data/basic.xbm")
//!     .map(BufReader::new)
//!     .unwrap();
//! let decoder = Decoder::new(reader).unwrap();
//! assert_eq!(decoder.width(), 8);
//! assert_eq!(decoder.height(), 7);
//!
//! let mut buf = [u8::default(); 56];
//! decoder.decode(&mut buf).unwrap();
//! assert_eq!(buf, *expected);
//! ```
//!
//! ### `image` crate support
//!
//! ```
//! # #[cfg(feature = "image")]
//! # {
//! use std::{
//!     fs::File,
//!     io::{BufReader, Cursor},
//! };
//!
//! use xbm::{
//!     Decoder,
//!     image::{DynamicImage, ImageDecoder, ImageFormat},
//! };
//!
//! let reader = File::open("tests/data/qr_code.xbm")
//!     .map(BufReader::new)
//!     .unwrap();
//! let decoder = Decoder::new(reader).unwrap();
//! assert_eq!(decoder.dimensions(), (296, 296));
//! let image = DynamicImage::from_decoder(decoder).unwrap();
//!
//! let mut writer = Cursor::new(Vec::with_capacity(2091));
//! image.write_to(&mut writer, ImageFormat::Png).unwrap();
//!
//! let actual = image::load_from_memory(writer.get_ref()).unwrap();
//! let expected = image::open("tests/data/qr_code.png").unwrap();
//! assert_eq!(actual, expected);
//! # }
//! ```
//!
//! [XBM]: https://en.wikipedia.org/wiki/X_BitMap
//! [X version 11 bitmap file format]: https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#Manipulating_Bitmaps

#![doc(html_root_url = "https://docs.rs/xbm/0.2.1/")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

pub mod decode;
pub mod encode;

#[cfg(feature = "image")]
pub use image;

pub use crate::{decode::Decoder, encode::Encoder};
