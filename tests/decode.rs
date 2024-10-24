// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
    error::Error as _,
    fs::File,
    io::{BufReader, Cursor},
    num::{IntErrorKind, ParseIntError},
};

use xbm::{decode::Error, Decoder};

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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
    decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
        decoder.decode(&mut buf).unwrap();
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
    decoder.decode(&mut buf).unwrap();
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
    decoder.decode(&mut buf).unwrap();
    assert_eq!(buf.as_slice(), expected);
}

#[test]
fn decode_with_invalid_width_statement() {
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
        let image = "#define image_width
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
        let image = "#define image_width 8 16
#define image_height 7
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
        let buf = Cursor::new(image);
        let err = Decoder::new(buf).unwrap_err();
        assert!(matches!(err, Error::InvalidHeader));
    }
}

#[test]
fn decode_with_invalid_height_statement() {
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
#define image_height
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
#define image_height 7 14
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
        let buf = Cursor::new(image);
        let err = Decoder::new(buf).unwrap_err();
        assert!(matches!(err, Error::InvalidHeader));
    }
}

#[test]
fn decode_with_invalid_x_hot_statement() {
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
#define image_x_hot
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
#define image_x_hot 4 8
#define image_y_hot 3
static unsigned char image_bits[] = {
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
        let buf = Cursor::new(image);
        let err = Decoder::new(buf).unwrap_err();
        assert!(matches!(err, Error::InvalidHeader));
    }
}

#[test]
fn decode_with_invalid_y_hot_statement() {
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
#define image_x_hot 4
#define image_y_hot
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
#define image_y_hot 3 6
static unsigned char image_bits[] = {
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
fn decode_with_invalid_array_declaration() {
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
    {
        let image = "#define image_width 8
#define image_height 7
static  unsigned  char  image_bits[]  =  {
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
static unsigned char image_bits[]={
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
static unsigned char image _bits[] = {
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
static unsigned char image_ bits[] = {
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
static unsigned char image_bits [] = {
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
static unsigned char image_bits() = {
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
static unsigned char image_bits[] + {
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
static unsigned char image_bits[] = [
    0x00, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
];
";
        let buf = Cursor::new(image);
        let err = Decoder::new(buf).unwrap_err();
        assert!(matches!(err, Error::InvalidHeader));
    }
}

#[test]
fn decode_with_invalid_width_value() {
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
}

#[test]
fn decode_with_invalid_height_value() {
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
}

#[test]
fn decode_with_invalid_x_hot_value() {
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
}

#[test]
fn decode_with_invalid_y_hot_value() {
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
    \u{1F980}, 0x1C, 0x24, 0x1C, 0x24, 0x1C, 0x00,
};
";
        let image = Cursor::new(image);
        let decoder = Decoder::new(image).unwrap();
        let mut buf = [u8::default(); 56];
        let err = decoder.decode(&mut buf).unwrap_err();
        if let Error::InvalidHexByte(value) = err {
            assert_eq!(value, "\u{1F980}");
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        decoder.decode(&mut buf).unwrap();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
        let err = decoder.decode(&mut buf).unwrap_err();
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
    let err = decoder.decode(&mut buf).unwrap_err();
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
    let _: Result<(), Error> = decoder.decode(&mut buf);
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
    let err = decoder.decode(&mut buf).unwrap_err();
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
    let _: Result<(), Error> = decoder.decode(&mut buf);
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
    decoder.read_image(&mut buf).unwrap();
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
    decoder.decode(&mut buf).unwrap();

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
