use std::borrow::Cow;
use std::str::Utf8Error;
use pgn_reader::RawHeader;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Like [`pgn_reader::RawHeader`], but has ownership of the bytes.
///
/// serde implementations are derived; they will use the inner byte vec.
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

    // CLIPPY: The linked docs cover it.
    #[allow(clippy::missing_errors_doc)]
    /// See [`pgn_reader::RawHeader::decode_utf8`].
    pub fn decode_utf8(&self) -> Result<Cow<str>, Utf8Error> {
        RawHeader(&self.0).decode_utf8()
    }

    /// See [`pgn_reader::RawHeader::decode_utf8_lossy`].
    pub fn decode_utf8_lossy(&self) -> Cow<str> {
        RawHeader(&self.0).decode_utf8_lossy()
    }
}