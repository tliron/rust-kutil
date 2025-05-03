use std::fmt;

/// Encodings in order from cheapest to decode to most expensive.
pub const ENCODINGS_BY_DECODING_COST: &[Encoding] =
    &[Encoding::Zstandard, Encoding::Deflate, Encoding::GZip, Encoding::Brotli];

//
// Encoding
//

/// HTTP encoding.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Encoding {
    /// Identity.
    #[default]
    Identity,

    /// Brotli.
    Brotli,

    /// Deflate.
    Deflate,

    /// GZip.
    GZip,

    /// Zstandard.
    Zstandard,
}

impl fmt::Display for Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
