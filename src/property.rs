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

#[allow(dead_code)]
/// Represents 7z compression methods.
#[derive(Debug)]
pub enum Method {
    Copy,
    Delta,
    Lzma2,
    Sz(Sz),
    Misc(Misc),
    Crypto(Crypto),
}

impl Method {
    #[allow(dead_code)]
    /// Gets the ID of this method.
    pub const fn id(&self) -> &'static [u8] {
        match self {
            Self::Copy => &[0x00],
            Self::Delta => &[0x03],
            Self::Lzma2 => &[0x21],
            Self::Sz(sz) => sz.id(),
            Self::Misc(misc) => misc.id(),
            Self::Crypto(crypto) => crypto.id(),
        }
    }

    #[allow(dead_code)]
    /// Gets the method associated with this ID.
    pub fn by_id(id: &[u8]) -> Option<Self> {
        match id {
            [0x00] => Some(Self::Copy),
            [0x03] => Some(Self::Delta),
            [0x21] => Some(Self::Lzma2),
            sz @ [0x03, ..] => Sz::by_id(sz).map(Self::Sz),
            misc @ [0x04, ..] => Misc::by_id(misc).map(Self::Misc),
            crypto @ [0x06, ..] => Crypto::by_id(crypto).map(Self::Crypto),
            _ => None,
        }
    }
}

#[allow(dead_code)]
/// Represents 7z methods.
#[derive(Debug)]
pub enum Sz {
    Lzma,
    Bcj,
    Bcj2,
    Ppc,
    Ia64,
    Arm,
    ArmT,
    Sparc,
    Ppmd,
}

impl Sz {
    #[allow(dead_code)]
    /// Gets the ID of this method.
    const fn id(&self) -> &'static [u8] {
        match self {
            Self::Lzma => &[0x03, 0x01, 0x01],
            Self::Bcj => &[0x03, 0x03, 0x01, 0x03],
            Self::Bcj2 => &[0x03, 0x03, 0x01, 0x1b],
            Self::Ppc => &[0x03, 0x03, 0x02, 0x05],
            Self::Ia64 => &[0x03, 0x03, 0x04, 0x01],
            Self::Arm => &[0x03, 0x03, 0x05, 0x01],
            Self::ArmT => &[0x03, 0x03, 0x07, 0x01],
            Self::Sparc => &[0x03, 0x03, 0x08, 0x05],
            Self::Ppmd => &[0x03, 0x04, 0x01],
        }
    }

    #[allow(dead_code)]
    /// Gets the method associated with this ID.
    const fn by_id(id: &[u8]) -> Option<Self> {
        match id {
            [0x03, 0x01, 0x01] => Some(Self::Lzma),
            [0x03, 0x03, 0x01, 0x03] => Some(Self::Bcj),
            [0x03, 0x03, 0x01, 0x1b] => Some(Self::Bcj2),
            [0x03, 0x03, 0x02, 0x05] => Some(Self::Ppc),
            [0x03, 0x03, 0x04, 0x01] => Some(Self::Ia64),
            [0x03, 0x03, 0x05, 0x01] => Some(Self::Arm),
            [0x03, 0x03, 0x07, 0x01] => Some(Self::ArmT),
            [0x03, 0x03, 0x08, 0x05] => Some(Self::Sparc),
            [0x03, 0x04, 0x01] => Some(Self::Ppmd),
            _ => None,
        }
    }
}

#[allow(dead_code)]
/// Represents misc methods.
#[derive(Debug)]
pub enum Misc {
    Deflate,
    Deflate64,
    Bzip2,
}

impl Misc {
    #[allow(dead_code)]
    /// Gets the ID of this method.
    const fn id(&self) -> &'static [u8] {
        match self {
            Self::Deflate => &[0x04, 0x01, 0x08],
            Self::Deflate64 => &[0x04, 0x01, 0x09],
            Self::Bzip2 => &[0x04, 0x02, 0x02],
        }
    }

    #[allow(dead_code)]
    /// Gets the method associated with this ID.
    const fn by_id(id: &[u8]) -> Option<Self> {
        match id {
            [0x04, 0x01, 0x08] => Some(Self::Deflate),
            [0x04, 0x01, 0x09] => Some(Self::Deflate64),
            [0x04, 0x02, 0x02] => Some(Self::Bzip2),
            _ => None,
        }
    }
}

#[allow(dead_code)]
/// Represents crypto methods.
#[derive(Debug)]
pub enum Crypto {
    SzAes,
}

impl Crypto {
    #[allow(dead_code)]
    /// Gets the ID of this method.
    const fn id(&self) -> &'static [u8] {
        match self {
            Self::SzAes => &[0x06, 0xf1, 0x07, 0x01],
        }
    }

    #[allow(dead_code)]
    /// Gets the method associated with this ID.
    const fn by_id(id: &[u8]) -> Option<Self> {
        match id {
            [0x06, 0xf1, 0x07, 0x01] => Some(Self::SzAes),
            _ => None,
        }
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

    #[test]
    fn get_method_id() {
        assert_eq!(Method::Copy.id(), [0x00]);
        assert_eq!(Method::Delta.id(), [0x03]);
        assert_eq!(Method::Lzma2.id(), [0x21]);
        assert_eq!(Method::Sz(Sz::Lzma).id(), [0x03, 0x01, 0x01]);
        assert_eq!(Method::Sz(Sz::Bcj).id(), [0x03, 0x03, 0x01, 0x03]);
        assert_eq!(Method::Sz(Sz::Bcj2).id(), [0x03, 0x03, 0x01, 0x1b]);
        assert_eq!(Method::Sz(Sz::Ppc).id(), [0x03, 0x03, 0x02, 0x05]);
        assert_eq!(Method::Sz(Sz::Ia64).id(), [0x03, 0x03, 0x04, 0x01]);
        assert_eq!(Method::Sz(Sz::Arm).id(), [0x03, 0x03, 0x05, 0x01]);
        assert_eq!(Method::Sz(Sz::ArmT).id(), [0x03, 0x03, 0x07, 0x01]);
        assert_eq!(Method::Sz(Sz::Sparc).id(), [0x03, 0x03, 0x08, 0x05]);
        assert_eq!(Method::Sz(Sz::Ppmd).id(), [0x03, 0x04, 0x01]);
        assert_eq!(Method::Misc(Misc::Deflate).id(), [0x04, 0x01, 0x08]);
        assert_eq!(Method::Misc(Misc::Deflate64).id(), [0x04, 0x01, 0x09]);
        assert_eq!(Method::Misc(Misc::Bzip2).id(), [0x04, 0x02, 0x02]);
        assert_eq!(Method::Crypto(Crypto::SzAes).id(), [0x06, 0xf1, 0x07, 0x01]);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn get_method_by_id() {
        assert!(matches!(Method::by_id(&[0x00]).unwrap(), Method::Copy));
        assert!(matches!(Method::by_id(&[0x03]).unwrap(), Method::Delta));
        assert!(matches!(Method::by_id(&[0x21]).unwrap(), Method::Lzma2));
        assert!(matches!(
            Method::by_id(&[0x03, 0x01, 0x01]).unwrap(),
            Method::Sz(Sz::Lzma)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x01, 0x03]).unwrap(),
            Method::Sz(Sz::Bcj)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x01, 0x1b]).unwrap(),
            Method::Sz(Sz::Bcj2)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x02, 0x05]).unwrap(),
            Method::Sz(Sz::Ppc)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x04, 0x01]).unwrap(),
            Method::Sz(Sz::Ia64)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x05, 0x01]).unwrap(),
            Method::Sz(Sz::Arm)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x07, 0x01]).unwrap(),
            Method::Sz(Sz::ArmT)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x03, 0x08, 0x05]).unwrap(),
            Method::Sz(Sz::Sparc)
        ));
        assert!(matches!(
            Method::by_id(&[0x03, 0x04, 0x01]).unwrap(),
            Method::Sz(Sz::Ppmd)
        ));
        assert!(matches!(
            Method::by_id(&[0x04, 0x01, 0x08]).unwrap(),
            Method::Misc(Misc::Deflate)
        ));
        assert!(matches!(
            Method::by_id(&[0x04, 0x01, 0x09]).unwrap(),
            Method::Misc(Misc::Deflate64)
        ));
        assert!(matches!(
            Method::by_id(&[0x04, 0x02, 0x02]).unwrap(),
            Method::Misc(Misc::Bzip2)
        ));
        assert!(matches!(
            Method::by_id(&[0x06, 0xf1, 0x07, 0x01]).unwrap(),
            Method::Crypto(Crypto::SzAes)
        ));
    }

    #[test]
    fn get_method_by_id_with_bad_id() {
        assert!(Method::by_id(&[u8::MAX]).is_none());
    }
}
