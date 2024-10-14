// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of converting a XBM file to a PNG file.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use image::{ColorType, ImageFormat};
use xbm::Decoder;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Input XBM file.
    #[arg(value_name("INFILE"))]
    input: PathBuf,

    /// Output PNG file.
    #[arg(value_name("OUTFILE"))]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let reader = File::open(&opt.input)
        .map(BufReader::new)
        .with_context(|| format!("could not open {}", opt.input.display()))?;
    let decoder = Decoder::new(reader).context("could not create new XBM decoder")?;
    let (width, height) = (decoder.width(), decoder.height());
    let mut buf =
        vec![u8::default(); usize::try_from(width).unwrap() * usize::try_from(height).unwrap()];
    decoder
        .decode(buf.as_mut_slice())
        .context("could not decode XBM image")?;

    buf.iter_mut()
        .for_each(|p| *p = if p == &0 { u8::MAX } else { u8::MIN });
    image::save_buffer_with_format(
        &opt.output,
        &buf,
        width,
        height,
        ColorType::L8,
        ImageFormat::Png,
    )
    .with_context(|| format!("could not save PNG image to {}", opt.output.display()))
}
