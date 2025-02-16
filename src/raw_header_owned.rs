use std::borrow::Cow;
use std::str::Utf8Error;
use pgn_reader::RawHeader;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Has functions of the [`pgn_reader::RawHeader`], but has ownership of the bytes.
pub struct RawHeaderOwned(pub Vec<u8>);

impl From<RawHeader<'_>> for RawHeaderOwned {
    fn from(raw_header: RawHeader) -> Self {
        Self(raw_header.0.to_vec())
    }
}

impl RawHeaderOwned {
    /// See [`pgn_reader::RawHeader::decode`].
    pub fn decode(&self) -> Cow<[u8]> {
        RawHeader(&self.0).decode()
    }

    /// See [`pgn_reader::RawHeader::decode_utf8`].
    pub fn decode_utf8(&self) -> Result<Cow<str>, Utf8Error> {
        RawHeader(&self.0).decode_utf8()
    }

    /// See [`pgn_reader::RawHeader::decode_utf8_lossy`].
    pub fn decode_utf8_lossy(&self) -> Cow<str> {
        RawHeader(&self.0).decode_utf8_lossy()
    }
}