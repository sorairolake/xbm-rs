// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

extern crate test;

use std::io::Write;

use image::DynamicImage;
use test::Bencher;
use xbm::Encoder;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| {
        let buf = [].as_mut_slice();
        Encoder::new(buf)
    });
}

#[bench]
fn encode(b: &mut Bencher) {
    let mut pixels = image::open("tests/data/qr_code.png")
        .map(DynamicImage::into_bytes)
        .unwrap();
    pixels.iter_mut().for_each(|p| *p = u8::from(*p < 128));
    let pixels = test::black_box(pixels);

    let mut buf = Vec::with_capacity(69460);

    b.iter(|| {
        let encoder = Encoder::new(buf.by_ref());
        encoder
            .encode(&pixels, "qr_code", 296, 296, None, None)
            .unwrap();
        buf.clear();
    });
}

#[cfg(feature = "image")]
#[bench]
fn write_image(b: &mut Bencher) {
    use image::{ExtendedColorType, ImageEncoder};

    let mut pixels = image::open("tests/data/qr_code.png")
        .map(DynamicImage::into_bytes)
        .unwrap();
    pixels.iter_mut().for_each(|p| *p = u8::from(*p < 128));
    let pixels = test::black_box(pixels);

    let mut buf = Vec::with_capacity(69460);

    b.iter(|| {
        let encoder = Encoder::new(buf.by_ref());
        encoder
            .write_image(&pixels, 296, 296, ExtendedColorType::L1)
            .unwrap();
        buf.clear();
    });
}
