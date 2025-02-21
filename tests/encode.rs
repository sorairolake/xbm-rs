// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io::{ErrorKind, Write},
    str,
};

use xbm::Encoder;

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
fn encode_width_name() {
    // "B" (8x7)
    let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x00\x00\x00\x00\x00\x00";

    let mut buf = [u8::default(); 129];
    let encoder = Encoder::new(buf.as_mut_slice());
    encoder.encode(pixels, "test", 8, 7, None, None).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), include_str!("data/name.xbm"));
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
    let err = encoder
        .encode(pixels, "image", 8, 7, None, None)
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(
        err.to_string(),
        "`buf` contains values other than `0` and `1`"
    );
}

#[test]
fn valid_name() {
    // "B" (8x7)
    let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x00\x00\x00\x00\x00\x00";

    let mut buf = Vec::with_capacity(144);

    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(encoder.encode(pixels, "A", 8, 7, None, None).is_ok());
        buf.clear();
    }
    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(encoder.encode(pixels, "a", 8, 7, None, None).is_ok());
        buf.clear();
    }
    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(encoder.encode(pixels, "TEST", 8, 7, None, None).is_ok());
        buf.clear();
    }
    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(encoder.encode(pixels, "test", 8, 7, None, None).is_ok());
        buf.clear();
    }
    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(encoder.encode(pixels, "C17", 8, 7, None, None).is_ok());
        buf.clear();
    }
    {
        let encoder = Encoder::new(buf.by_ref());
        assert!(
            encoder
                .encode(pixels, "\u{30C6}\u{30B9}\u{30C8}", 8, 7, None, None)
                .is_ok()
        );
    }
}

#[test]
fn invalid_name() {
    // "B" (8x7)
    let pixels = b"\x00\x00\x00\x00\x00\x00\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x01\x00\x00\x01\x00\x00\
                   \x00\x00\x01\x01\x01\x00\x00\x00\
                   \x00\x00\x00\x00\x00\x00\x00\x00";

    let mut buf = [];

    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder.encode(pixels, "", 8, 7, None, None).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder.encode(pixels, "0", 8, 7, None, None).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder.encode(pixels, "_", 8, 7, None, None).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder.encode(pixels, " ", 8, 7, None, None).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder
            .encode(pixels, "ANSI C", 8, 7, None, None)
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder
            .encode(pixels, "XBM\0", 8, 7, None, None)
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
    {
        let encoder = Encoder::new(buf.as_mut_slice());
        let err = encoder
            .encode(pixels, "\u{1F980}", 8, 7, None, None)
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "invalid C identifier prefix");
    }
}

#[test]
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
    let err = encoder
        .encode(pixels, "image", 8, 7, Some(4), None)
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "only one of `x_hot` and `y_hot` is `Some`");
}

#[test]
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
    let err = encoder
        .encode(pixels, "image", 8, 7, None, Some(3))
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "only one of `x_hot` and `y_hot` is `Some`");
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
    let _ = encoder.encode(pixels, "image", 4, 3, None, None);
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
    let input = image::open("tests/data/qr_code.png").unwrap();

    let mut buf = Vec::with_capacity(69454);
    let encoder = Encoder::new(buf.by_ref());
    input.write_with_encoder(encoder).unwrap();
    assert_eq!(
        str::from_utf8(&buf).unwrap(),
        include_str!("data/qr_code.xbm")
    );
}
