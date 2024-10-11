// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `xbm` crate is a [XBM] encoding and decoding library.
//!
//! This crate supports the [X version 11 bitmap file format].
//!
//! [XBM]: https://en.wikipedia.org/wiki/X_BitMap
//! [X version 11 bitmap file format]: https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#Manipulating_Bitmaps

#![doc(html_root_url = "https://docs.rs/xbm/0.1.0/")]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[allow(missing_docs)]
#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
