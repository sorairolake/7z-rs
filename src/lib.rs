//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021 Shun Sakai
//

//! A library for reading/writing the [7z format][7z].
//!
//! The implementation is based on the format specification contained in the
//! [LZMA SDK][sdk] v21.07.
//!
//! [7z]: https://www.7-zip.org/7z.html
//! [sdk]: https://www.7-zip.org/sdk.html

#![doc(html_root_url = "https://docs.rs/sz/0.0.1/sz/")]
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]

pub mod filetime;

pub use crate::filetime::FileTime;
