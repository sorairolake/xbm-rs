// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::str;

use xbm::{encode::Error, Encoder};

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
        include_str!("data/basic.xbm")
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
        include_str!("data/16x14.xbm")
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
        include_str!("data/width_7.xbm")
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
        include_str!("data/width_14.xbm")
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
        include_str!("data/hotspot.xbm")
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
        include_str!("data/basic.xbm")
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
        include_str!("data/basic.xbm")
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
    use std::io::Write;

    let input = image::open("tests/data/qr_code.png").unwrap();
    let (width, height) = (input.width(), input.height());
    let mut pixels = input.into_bytes();
    pixels.iter_mut().for_each(|p| *p = u8::from(*p < 128));

    let mut buf = Vec::with_capacity(69460);
    let encoder = Encoder::new(buf.by_ref());
    encoder
        .encode(pixels, "qr_code", width, height, None, None)
        .unwrap();
    assert_eq!(
        str::from_utf8(&buf).unwrap(),
        include_str!("data/qr_code.xbm")
    );
}