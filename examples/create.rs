//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! An example of creating an archive from files.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use clap::Parser;

/// Create an archive from files.
#[derive(Debug, Parser)]
#[clap(version, about)]
struct Opt {
    /// An archive to create.
    pub archive: std::path::PathBuf,

    /// Files to add to the archive.
    pub file: Vec<std::path::PathBuf>,
}

fn main() {
    #[allow(unused_variables)]
    let opt = Opt::parse();

    todo!();
}
