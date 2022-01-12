//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021 Shun Sakai
//

//! An example of creating an archive from files.

use std::path::PathBuf;

use clap::Parser;

/// Create an archive from files.
#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// An archive to create.
    pub archive: PathBuf,

    /// Files to add to the archive.
    pub file: Vec<PathBuf>,
}

fn main() {
    #[allow(unused_variables)]
    let opt = Opt::parse();

    todo!();
}
