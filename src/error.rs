//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! The error type for this crate.

use std::{borrow::Cow, io};

use thiserror::Error;

use crate::property::Property;

/// The error type for the 7z format.
#[derive(Debug, Error)]
pub enum Error {
    /// The archive was invalid.
    #[error("invalid 7z archive")]
    InvalidArchive(#[from] InvalidArchive),

    /// An error caused by I/O.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// The password was required to decrypt the archive.
    #[error("password required to decrypt 7z archive")]
    PasswordRequired,

    /// The archive was not supported.
    #[error("unsupported 7z archive")]
    UnsupportedArchive(#[from] UnsupportedArchive),

    /// A custom error.
    #[error("{0}")]
    Other(Cow<'static, str>),
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        Self::new(io::ErrorKind::Other, error)
    }
}

/// The error type if the archive was invalid.
#[derive(Debug, Error)]
pub enum InvalidArchive {
    /// The signature was invalid.
    #[error("invalid signature `{0:02x?}`")]
    Signature([u8; 6]),

    /// A CRC of the start header mismatched.
    #[error("start header CRC mismatch")]
    StartHeaderCrc,

    /// A CRC of the next header mismatched.
    #[error("next header CRC mismatch")]
    NextHeaderCrc,

    /// The end property was invalid.
    #[error("expected end id for {pos:?}, found `{id:#04x}`")]
    EndProperty {
        /// The position of property where the end property was invalid.
        pos: Property,

        /// The id which was found.
        id: u8,
    },
}

/// The error type if the archive was not supported.
#[derive(Debug, Error)]
pub enum UnsupportedArchive {
    /// The version was not supported.
    #[error("unsupported version `{major}.{minor}`")]
    Version {
        /// Major version.
        major: u8,

        /// Minor version.
        minor: u8,
    },

    /// The compression method was not supported.
    #[error("unsupported compression method `{0}`")]
    CompressionMethod(String),
}

/// Alias for a [`Result`](std::result::Result) with [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    #[test]
    fn display_invalid_archive_error() {
        let invalid_signature_error =
            Error::InvalidArchive(InvalidArchive::Signature(Default::default()));
        assert_eq!(format!("{invalid_signature_error}"), "invalid 7z archive");

        assert_eq!(
            format!("{}", invalid_signature_error.source().unwrap()),
            "invalid signature `[00, 00, 00, 00, 00, 00]`"
        );
        assert_eq!(
            format!(
                "{}",
                Error::InvalidArchive(InvalidArchive::StartHeaderCrc)
                    .source()
                    .unwrap()
            ),
            "start header CRC mismatch"
        );
        assert_eq!(
            format!(
                "{}",
                Error::InvalidArchive(InvalidArchive::NextHeaderCrc)
                    .source()
                    .unwrap()
            ),
            "next header CRC mismatch"
        );
        assert_eq!(
            format!(
                "{}",
                Error::InvalidArchive(InvalidArchive::EndProperty {
                    pos: Property::Header,
                    id: u8::default()
                })
                .source()
                .unwrap()
            ),
            "expected end id for Header, found `0x00`"
        );
    }

    #[test]
    fn display_io_error() {
        assert_eq!(
            format!("{}", Error::Io(io::ErrorKind::NotFound.into())),
            "entity not found"
        );
    }

    #[test]
    fn display_password_required_error() {
        assert_eq!(
            format!("{}", Error::PasswordRequired),
            "password required to decrypt 7z archive"
        );
    }

    #[test]
    fn display_unsupported_archive_error() {
        let unsupported_version_error = Error::UnsupportedArchive(UnsupportedArchive::Version {
            major: u8::default(),
            minor: u8::default(),
        });
        assert_eq!(
            format!("{unsupported_version_error}"),
            "unsupported 7z archive"
        );

        assert_eq!(
            format!("{}", unsupported_version_error.source().unwrap()),
            "unsupported version `0.0`"
        );
        assert_eq!(
            format!(
                "{}",
                Error::UnsupportedArchive(UnsupportedArchive::CompressionMethod(
                    "LZMA".to_string()
                ))
                .source()
                .unwrap()
            ),
            "unsupported compression method `LZMA`"
        );
    }

    #[test]
    fn display_other_error() {
        assert_eq!(format!("{}", Error::Other("Error".into())), "Error");
    }

    #[test]
    fn source_invalid_archive_error() {
        assert!(
            Error::InvalidArchive(InvalidArchive::Signature(Default::default()))
                .source()
                .unwrap()
                .is::<InvalidArchive>()
        );
        assert!(Error::InvalidArchive(InvalidArchive::StartHeaderCrc)
            .source()
            .unwrap()
            .is::<InvalidArchive>());
        assert!(Error::InvalidArchive(InvalidArchive::NextHeaderCrc)
            .source()
            .unwrap()
            .is::<InvalidArchive>());
        assert!(Error::InvalidArchive(InvalidArchive::EndProperty {
            pos: Property::Header,
            id: u8::default()
        })
        .source()
        .unwrap()
        .is::<InvalidArchive>());
    }

    #[test]
    fn source_io_error() {
        assert!(Error::Io(io::ErrorKind::NotFound.into()).source().is_none());
    }

    #[test]
    fn source_password_required_error() {
        assert!(Error::PasswordRequired.source().is_none());
    }

    #[test]
    fn source_unsupported_archive_error() {
        assert!(Error::UnsupportedArchive(UnsupportedArchive::Version {
            major: u8::default(),
            minor: u8::default()
        })
        .source()
        .unwrap()
        .is::<UnsupportedArchive>());
        assert!(
            Error::UnsupportedArchive(UnsupportedArchive::CompressionMethod("LZMA".to_string()))
                .source()
                .unwrap()
                .is::<UnsupportedArchive>()
        );
    }

    #[test]
    fn source_other_error() {
        assert!(Error::Other("Error".into()).source().is_none());
    }

    #[test]
    fn invalid_archive_error_to_error() {
        assert!(matches!(
            Error::from(InvalidArchive::Signature(Default::default())),
            Error::InvalidArchive(InvalidArchive::Signature(_))
        ));
        assert!(matches!(
            Error::from(InvalidArchive::StartHeaderCrc),
            Error::InvalidArchive(InvalidArchive::StartHeaderCrc)
        ));
        assert!(matches!(
            Error::from(InvalidArchive::NextHeaderCrc),
            Error::InvalidArchive(InvalidArchive::NextHeaderCrc)
        ));
        assert!(matches!(
            Error::from(InvalidArchive::EndProperty {
                pos: Property::Header,
                id: u8::default()
            }),
            Error::InvalidArchive(InvalidArchive::EndProperty { .. })
        ));
    }

    #[test]
    fn error_to_std_io_error() {
        assert!(matches!(
            io::Error::from(Error::Io(io::ErrorKind::NotFound.into())),
            io::Error { .. }
        ));
    }

    #[test]
    fn std_io_error_to_error() {
        assert!(matches!(
            Error::from(io::Error::from(io::ErrorKind::NotFound)),
            Error::Io(_)
        ));
    }

    #[test]
    fn unsupported_archive_error_to_error() {
        assert!(matches!(
            Error::from(UnsupportedArchive::Version {
                major: u8::default(),
                minor: u8::default()
            }),
            Error::UnsupportedArchive(UnsupportedArchive::Version { .. })
        ));
        assert!(matches!(
            Error::from(UnsupportedArchive::CompressionMethod("LZMA".to_string())),
            Error::UnsupportedArchive(UnsupportedArchive::CompressionMethod(_))
        ));
    }

    #[test]
    fn result_type() {
        assert_eq!(
            std::any::type_name::<Result<()>>(),
            std::any::type_name::<std::result::Result<(), Error>>()
        );
    }
}
