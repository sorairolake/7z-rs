//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021 Shun Sakai
//

//! 7z timestamp.

use std::time::SystemTime;

use thiserror::Error;
use time::{macros::datetime, Duration, OffsetDateTime};

/// The NT time epoch.
const NT_EPOCH: OffsetDateTime = datetime!(1601-01-01 00:00 UTC);

/// The error type for 7z timestamp.
#[derive(Debug, Error)]
pub enum Error {
    /// Out of range of the timestamp.
    #[error("Out of range of 7z timestamp")]
    InvalidFileTime,

    /// The timestamp is too big.
    #[error("7z timestamp is too big")]
    FileTimeTooBig,
}

/// Represents 7z timestamp.
///
/// This is the same as the [Windows NT system time][file-times].
///
/// [file-times]: https://docs.microsoft.com/en-us/windows/win32/sysinfo/file-times
#[derive(Debug, PartialEq)]
pub struct FileTime(u64);

impl From<u64> for FileTime {
    /// Convert the Windows NT system time to [`FileTime`].
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<SystemTime> for FileTime {
    type Error = Error;

    /// Convert [`SystemTime`] to [`FileTime`].
    ///
    /// # Errors
    ///
    /// This function will return an error if `value` is out of range of the
    /// Windows NT system time.
    fn try_from(value: SystemTime) -> Result<Self, Self::Error> {
        Self::try_from(OffsetDateTime::from(value))
    }
}

impl TryFrom<OffsetDateTime> for FileTime {
    type Error = Error;

    /// Convert [`OffsetDateTime`] to [`FileTime`].
    ///
    /// # Errors
    ///
    /// This function will return an error if `value` is out of range of the
    /// Windows NT system time.
    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        let elapsed = (value - NT_EPOCH).whole_nanoseconds();

        match u64::try_from(elapsed / 100) {
            Ok(ft) if !elapsed.is_negative() => Ok(Self(ft)),
            _ => Err(Error::InvalidFileTime),
        }
    }
}

impl From<FileTime> for u64 {
    /// Convert [`FileTime`] to the Windows NT system time.
    fn from(value: FileTime) -> Self {
        value.0
    }
}

impl TryFrom<FileTime> for SystemTime {
    type Error = Error;

    /// Convert [`FileTime`] to [`SystemTime`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the `large-dates` feature is
    /// disabled and `value` is out of range of [`OffsetDateTime`].
    fn try_from(value: FileTime) -> Result<Self, Self::Error> {
        let dt = OffsetDateTime::try_from(value)?;

        Ok(Self::from(dt))
    }
}

impl TryFrom<FileTime> for OffsetDateTime {
    type Error = Error;

    /// Convert [`FileTime`] to [`OffsetDateTime`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the `large-dates` feature is
    /// disabled and `value` is out of range of [`OffsetDateTime`].
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn try_from(value: FileTime) -> Result<Self, Self::Error> {
        let duration = Duration::new(
            (value.0 / 10_000_000) as i64,
            ((value.0 % 10_000_000) * 100) as i32,
        );

        NT_EPOCH.checked_add(duration).ok_or(Error::FileTimeTooBig)
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::datetime, Duration, OffsetDateTime};

    use super::*;

    /// The largest value that can be represented by the Windows NT system time.
    #[cfg(feature = "large-dates")]
    const MAX: OffsetDateTime = datetime!(+60056-05-28 05:36:10.955_161_500 UTC);

    #[test]
    fn offset_date_time_to_file_time() {
        assert!(FileTime::try_from(NT_EPOCH - Duration::NANOSECOND).is_err());

        assert_eq!(FileTime::try_from(NT_EPOCH).unwrap(), FileTime(u64::MIN));
        assert_eq!(
            FileTime::try_from(OffsetDateTime::UNIX_EPOCH).unwrap(),
            FileTime(116_444_736_000_000_000)
        );
        assert_eq!(
            FileTime::try_from(datetime!(9999-12-31 23:59:59.999_999_999 UTC)).unwrap(),
            FileTime(2_650_467_743_999_999_999)
        );

        #[cfg(feature = "large-dates")]
        assert_eq!(FileTime::try_from(MAX).unwrap(), FileTime(u64::MAX));

        #[cfg(feature = "large-dates")]
        assert!(FileTime::try_from(MAX + Duration::nanoseconds(100)).is_err());
    }

    #[test]
    fn file_time_to_offset_date_time() {
        assert_eq!(
            OffsetDateTime::try_from(FileTime(u64::MIN)).unwrap(),
            NT_EPOCH
        );
        assert_eq!(
            OffsetDateTime::try_from(FileTime(116_444_736_000_000_000)).unwrap(),
            OffsetDateTime::UNIX_EPOCH
        );
        assert_eq!(
            OffsetDateTime::try_from(FileTime(2_650_467_743_999_999_999)).unwrap(),
            datetime!(9999-12-31 23:59:59.999_999_900 UTC)
        );

        #[cfg(not(feature = "large-dates"))]
        assert!(OffsetDateTime::try_from(FileTime(2_650_467_744_000_000_000)).is_err());

        #[cfg(feature = "large-dates")]
        assert_eq!(OffsetDateTime::try_from(FileTime(u64::MAX)).unwrap(), MAX);
    }
}
