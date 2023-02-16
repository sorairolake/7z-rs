//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! An example of extracting files in an archive.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use clap::Parser;

/// Extract files in an archive.
#[derive(Debug, Parser)]
#[clap(version, about)]
struct Opt {
    /// An archive to extract.
    pub archive: std::path::PathBuf,

    /// Entries to extract.
    ///
    /// If this is omitted, all entries will be extracted.
    pub entry: Vec<std::path::PathBuf>,
}

fn main() {
    #[allow(unused_variables)]
    let opt = Opt::parse();

    todo!();
}
