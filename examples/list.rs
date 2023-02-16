//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! An example of listing entries in an archive.

use std::path::PathBuf;

use clap::Parser;

/// List entries in an archive.
#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// An archive to list entries.
    pub archive: PathBuf,
}

fn main() {
    #[allow(unused_variables)]
    let opt = Opt::parse();

    todo!();
}
