// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use std::{fs::File, io::BufReader};

use test::Bencher;
use xbm::Decoder;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| {
        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        Decoder::new(reader).unwrap()
    });
}

#[bench]
fn decode(b: &mut Bencher) {
    let mut buf = test::black_box(vec![u8::default(); 87616]);

    b.iter(|| {
        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        decoder.decode(&mut buf).unwrap();
        buf.fill_with(u8::default);
    });
}

#[bench]
fn decode_to_vec(b: &mut Bencher) {
    b.iter(|| {
        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        decoder.decode_to_vec().unwrap()
    });
}

#[bench]
fn width(b: &mut Bencher) {
    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.width());
}

#[bench]
fn height(b: &mut Bencher) {
    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.height());
}

#[bench]
fn x_hot(b: &mut Bencher) {
    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.x_hot());
}

#[bench]
fn y_hot(b: &mut Bencher) {
    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.y_hot());
}

#[cfg(feature = "image")]
#[bench]
fn read_image(b: &mut Bencher) {
    use image::ImageDecoder;

    let mut buf = test::black_box(vec![u8::default(); 87616]);

    b.iter(|| {
        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        let decoder = Decoder::new(reader).unwrap();
        decoder.read_image(&mut buf).unwrap();
        buf.fill_with(u8::default);
    });
}

#[cfg(feature = "image")]
#[bench]
fn dimensions(b: &mut Bencher) {
    use image::ImageDecoder;

    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.dimensions());
}

#[cfg(feature = "image")]
#[bench]
fn color_type(b: &mut Bencher) {
    use image::ImageDecoder;

    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.color_type());
}

#[cfg(feature = "image")]
#[bench]
fn original_color_type(b: &mut Bencher) {
    use image::ImageDecoder;

    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.original_color_type());
}

#[cfg(feature = "image")]
#[bench]
fn icc_profile(b: &mut Bencher) {
    use image::ImageDecoder;

    b.iter(|| {
        let reader = File::open("tests/data/qr_code.xbm")
            .map(BufReader::new)
            .unwrap();
        let mut decoder = Decoder::new(reader).unwrap();
        decoder.icc_profile().unwrap()
    });
}

#[cfg(feature = "image")]
#[bench]
fn total_bytes(b: &mut Bencher) {
    use image::ImageDecoder;

    let reader = File::open("tests/data/qr_code.xbm")
        .map(BufReader::new)
        .unwrap();
    let decoder = Decoder::new(reader).unwrap();

    b.iter(|| decoder.total_bytes());
}
