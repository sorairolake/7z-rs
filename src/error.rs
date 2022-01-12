//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021 Shun Sakai
//

//! The error type for this crate.

use std::io;

use thiserror::Error;

/// Represents the error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// The archive is invalid.
    #[error("Invalid 7z archive")]
    InvalidArchive(String),

    /// An error caused by I/O.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// The password is required to decrypt the archive.
    #[error("The password is required to decrypt 7z archive")]
    PasswordRequired,

    /// The archive is not supported.
    #[error("Unsupported 7z archive")]
    UnsupportedArchive(String),
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        Self::new(io::ErrorKind::Other, error)
    }
}

/// Alias for a [`Result`](std::result::Result) with [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;
