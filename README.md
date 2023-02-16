# 7z-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**7z-rs** ([`sz`][version-url]) is a library for reading/writing the
[7z format][7z-format-url] in the
[Rust Programming Language][rust-official-url].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sz = "0.0.1"
```

### Crate features

#### `time`

Enables the [`time`][time-crate-url] crate.
This is enabled by default.

#### `large-dates`

Enables the `large-dates` feature of the [`time`][time-crate-url] crate.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.62.0.

## Changelog

Please see [CHANGELOG.adoc](CHANGELOG.adoc).

## Contributing

Please see [CONTRIBUTING.adoc](CONTRIBUTING.adoc).

## License

Copyright &copy; 2021&ndash;2023 Shun Sakai (see [AUTHORS.adoc](AUTHORS.adoc))

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

See [COPYRIGHT](COPYRIGHT), [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for more details.

[ci-badge]: https://github.com/sorairolake/7z-rs/workflows/CI/badge.svg
[ci-url]: https://github.com/sorairolake/7z-rs/actions?query=workflow%3ACI
[version-badge]: https://img.shields.io/crates/v/sz
[version-url]: https://crates.io/crates/sz
[docs-badge]: https://img.shields.io/docsrs/sz
[docs-url]: https://docs.rs/sz
[license-badge]: https://img.shields.io/crates/l/sz
[7z-format-url]: https://www.7-zip.org/7z.html
[rust-official-url]: https://www.rust-lang.org/
[time-crate-url]: https://crates.io/crates/time
