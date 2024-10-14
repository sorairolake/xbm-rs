<!--
SPDX-FileCopyrightText: 2024 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# xbm-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**xbm-rs** ([`xbm`][version-url]) is a [XBM] encoding and decoding library in
pure [Rust].

This crate supports the [X version 11 bitmap file format].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xbm = "0.1.0"
```

### Crate features

#### `image`

Enables the [`image`] crate support.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.74.0.

## Source code

The upstream repository is available at
<https://github.com/sorairolake/xbm-rs.git>.

The source code is also available at:

- <https://gitlab.com/sorairolake/xbm-rs.git>
- <https://codeberg.org/sorairolake/xbm-rs.git>

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright &copy; 2024 Shun Sakai (see [AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.2 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/xbm-rs/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/xbm-rs/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/xbm?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/xbm
[msrv-badge]: https://img.shields.io/crates/msrv/xbm?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/xbm?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/xbm
[license-badge]: https://img.shields.io/crates/l/xbm?style=for-the-badge
[XBM]: https://en.wikipedia.org/wiki/X_BitMap
[Rust]: https://www.rust-lang.org/
[X version 11 bitmap file format]: https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#Manipulating_Bitmaps
[`image`]: https://crates.io/crates/image
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
