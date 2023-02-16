//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! A library for reading/writing the [7z format][7z].
//!
//! The implementation is based on the format specification contained in the
//! [LZMA SDK][sdk] v21.07.
//!
//! # Features
//!
//! ## Default features
//!
//! - `time`: Enable the [`time`][time] crate.
//!
//! ## Optional features
//!
//! - `large-dates`: Enable the `large-dates` feature of the [`time`][time]
//!   crate.
//!
//! [7z]: https://www.7-zip.org/7z.html
//! [sdk]: https://www.7-zip.org/sdk.html
//! [time]: https://docs.rs/time

#![doc(html_root_url = "https://docs.rs/sz/0.0.1/sz/")]
// Lint levels of rustc.
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod error;
pub mod filetime;

pub use crate::error::{Error, Result};
pub use crate::filetime::FileTime;
