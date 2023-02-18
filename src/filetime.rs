//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! 7z timestamp.

#[cfg(feature = "time")]
use std::time::SystemTime;

use thiserror::Error;
#[cfg(feature = "time")]
use time::{macros::datetime, Duration, OffsetDateTime};

/// The NT time epoch.
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
const NT_EPOCH: OffsetDateTime = datetime!(1601-01-01 00:00 UTC);

/// The error type for 7z timestamp.
#[derive(Debug, Error)]
pub enum Error {
    /// Out of range of the timestamp.
    #[error("out of range of 7z timestamp")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileTime(u64);

impl Default for FileTime {
    /// Returns the default value of "1601-01-01 00:00:00 UTC".
    fn default() -> Self {
        Self(u64::MIN)
    }
}

impl From<u64> for FileTime {
    /// Converts the Windows NT system time to a `FileTime`.
    fn from(time: u64) -> Self {
        Self(time)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
impl TryFrom<SystemTime> for FileTime {
    type Error = Error;

    /// Converts a [`SystemTime`] to a `FileTime`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `time` is out of range of the Windows NT system time.
    fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
        Self::try_from(OffsetDateTime::from(time))
    }
}

#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
impl TryFrom<OffsetDateTime> for FileTime {
    type Error = Error;

    /// Converts a [`OffsetDateTime`] to a `FileTime`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `dt` is out of range of the Windows NT system time.
    fn try_from(dt: OffsetDateTime) -> Result<Self, Self::Error> {
        let elapsed = (dt - NT_EPOCH).whole_nanoseconds();
        match u64::try_from(elapsed / 100) {
            Ok(ft) if !elapsed.is_negative() => Ok(Self(ft)),
            _ => Err(Error::InvalidFileTime),
        }
    }
}

impl From<FileTime> for u64 {
    /// Converts a `FileTime` to the Windows NT system time.
    fn from(time: FileTime) -> Self {
        time.0
    }
}

#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
impl TryFrom<FileTime> for SystemTime {
    type Error = Error;

    /// Converts a `FileTime` to a [`SystemTime`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the `large-dates` feature is disabled and `time` is
    /// out of range of [`OffsetDateTime`].
    fn try_from(time: FileTime) -> Result<Self, Self::Error> {
        let dt = OffsetDateTime::try_from(time)?;
        Ok(Self::from(dt))
    }
}

#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
impl TryFrom<FileTime> for OffsetDateTime {
    type Error = Error;

    /// Converts a `FileTime` to a [`OffsetDateTime`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the `large-dates` feature is disabled and `time` is
    /// out of range of [`OffsetDateTime`].
    fn try_from(time: FileTime) -> Result<Self, Self::Error> {
        let duration = Duration::new(
            i64::try_from(time.0 / 10_000_000)
                .expect("the number of seconds is out of range of `i64`"),
            i32::try_from((time.0 % 10_000_000) * 100)
                .expect("the number of nanoseconds is out of range of `i32`"),
        );
        NT_EPOCH.checked_add(duration).ok_or(Error::FileTimeTooBig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The largest value that can be represented by the Windows NT system time.
    #[cfg(feature = "large-dates")]
    const MAX: OffsetDateTime = datetime!(+60056-05-28 05:36:10.955_161_500 UTC);

    #[test]
    fn default_file_time() {
        assert_eq!(FileTime::default(), FileTime(u64::MIN));
    }

    #[test]
    fn u64_to_file_time() {
        assert_eq!(FileTime::from(u64::MIN), FileTime(u64::MIN));
        assert_eq!(FileTime::from(u64::MAX), FileTime(u64::MAX));
    }

    #[cfg(feature = "time")]
    #[test]
    fn system_time_to_file_time() {
        assert_eq!(
            FileTime::try_from(std::time::UNIX_EPOCH).unwrap(),
            FileTime(116_444_736_000_000_000)
        );
    }

    #[cfg(feature = "time")]
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
    fn file_time_to_u64() {
        assert_eq!(u64::from(FileTime(u64::MIN)), u64::MIN);
        assert_eq!(u64::from(FileTime(u64::MAX)), u64::MAX);
    }

    #[cfg(feature = "time")]
    #[test]
    fn file_time_to_system_time() {
        assert_eq!(
            SystemTime::try_from(FileTime(116_444_736_000_000_000)).unwrap(),
            std::time::UNIX_EPOCH
        );
    }

    #[cfg(feature = "time")]
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
