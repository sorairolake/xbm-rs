// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of converting a XBM file to a PNG file.

use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use image::{DynamicImage, ImageFormat};
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
    let image = DynamicImage::from_decoder(decoder).context("could not decode XBM image")?;

    image
        .save_with_format(&opt.output, ImageFormat::Png)
        .with_context(|| format!("could not save PNG image to {}", opt.output.display()))
}
