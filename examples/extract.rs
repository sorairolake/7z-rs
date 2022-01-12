//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021 Shun Sakai
//

//! An example of extracting files in an archive.

use std::path::PathBuf;

use clap::Parser;

/// Extract files in an archive.
#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    /// An archive to extract.
    pub archive: PathBuf,

    /// Entries to extract.
    ///
    /// If this is omitted, all entries will be extracted.
    pub entry: Vec<PathBuf>,
}

fn main() {
    #[allow(unused_variables)]
    let opt = Opt::parse();

    todo!();
}
