//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2021-2023 Shun Sakai
//

//! 7z properties.

#[allow(dead_code)]
/// Magic number of the 7z format.
pub const MAGIC_NUMBER: [u8; 6] = [0x37, 0x7a, 0xbc, 0xaf, 0x27, 0x1c];

/// Represents 7z properties.
#[derive(Debug)]
pub enum Property {
    End,
    Header,
    ArchiveProperties,
    AdditionalStreamsInfo,
    MainStreamsInfo,
    FilesInfo,
    PackInfo,
    UnpackInfo,
    SubStreamsInfo,
    Size,
    Crc,
    Folder,
    CodersUnpackSize,
    NumUnpackStream,
    EmptyStream,
    EmptyFile,
    Anti,
    Name,
    CTime,
    ATime,
    MTime,
    WinAttributes,
    Comment,
    EncodedHeader,
    StartPos,
    Dummy,
}

impl From<Property> for u8 {
    fn from(property: Property) -> Self {
        property as Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_number() {
        assert_eq!(MAGIC_NUMBER, [b'7', b'z', 0xbc, 0xaf, 0x27, 0x1c]);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn property() {
        assert_eq!(u8::from(Property::End), 0x00);
        assert_eq!(u8::from(Property::Header), 0x01);
        assert_eq!(u8::from(Property::ArchiveProperties), 0x02);
        assert_eq!(u8::from(Property::AdditionalStreamsInfo), 0x03);
        assert_eq!(u8::from(Property::MainStreamsInfo), 0x04);
        assert_eq!(u8::from(Property::FilesInfo), 0x05);
        assert_eq!(u8::from(Property::PackInfo), 0x06);
        assert_eq!(u8::from(Property::UnpackInfo), 0x07);
        assert_eq!(u8::from(Property::SubStreamsInfo), 0x08);
        assert_eq!(u8::from(Property::Size), 0x09);
        assert_eq!(u8::from(Property::Crc), 0x0a);
        assert_eq!(u8::from(Property::Folder), 0x0b);
        assert_eq!(u8::from(Property::CodersUnpackSize), 0x0c);
        assert_eq!(u8::from(Property::NumUnpackStream), 0x0d);
        assert_eq!(u8::from(Property::EmptyStream), 0x0e);
        assert_eq!(u8::from(Property::EmptyFile), 0x0f);
        assert_eq!(u8::from(Property::Anti), 0x10);
        assert_eq!(u8::from(Property::Name), 0x11);
        assert_eq!(u8::from(Property::CTime), 0x12);
        assert_eq!(u8::from(Property::ATime), 0x13);
        assert_eq!(u8::from(Property::MTime), 0x14);
        assert_eq!(u8::from(Property::WinAttributes), 0x15);
        assert_eq!(u8::from(Property::Comment), 0x16);
        assert_eq!(u8::from(Property::EncodedHeader), 0x17);
        assert_eq!(u8::from(Property::StartPos), 0x18);
        assert_eq!(u8::from(Property::Dummy), 0x19);
    }
}
