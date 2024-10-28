// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of converting a PNG file to a XBM file.

use std::{fs::File, io::BufWriter, path::PathBuf};

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
        .map(DynamicImage::into_luma8)
        .map(DynamicImage::from)
        .with_context(|| format!("could not open {}", opt.input.display()))?;

    let writer = File::create(&opt.output)
        .map(BufWriter::new)
        .with_context(|| format!("could not open {}", opt.output.display()))?;
    let encoder = Encoder::new(writer);
    input
        .write_with_encoder(encoder)
        .context("could not encode to XBM image")
}
