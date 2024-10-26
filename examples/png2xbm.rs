// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of converting a PNG file to a XBM file.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{ffi::OsStr, fs::File, io::BufWriter, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use image::DynamicImage;
use xbm::Encoder;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Input PNG file.
    #[arg(value_name("INFILE"))]
    input: PathBuf,

    /// Output XBM file.
    #[arg(value_name("OUTFILE"))]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let input = image::open(&opt.input)
        .with_context(|| format!("could not open {}", opt.input.display()))?;
    let (width, height) = (input.width(), input.height());
    let mut input = DynamicImage::ImageLuma8(input.into_luma8()).into_bytes();
    input.iter_mut().for_each(|p| *p = u8::from(*p < 128));

    let writer = File::create(&opt.output)
        .map(BufWriter::new)
        .with_context(|| format!("could not open {}", opt.output.display()))?;
    let encoder = Encoder::new(writer);
    encoder
        .encode(
            input,
            opt.input
                .file_stem()
                .map_or("image".into(), OsStr::to_string_lossy),
            width,
            height,
            None,
            None,
        )
        .context("could not encode to XBM image")
}
