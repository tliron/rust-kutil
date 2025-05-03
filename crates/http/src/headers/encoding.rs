use super::{into::*, parseable::*};

use {http::header::*, kutil_transcoding::*, std::fmt};

impl IntoHeaderValue for Encoding {
    fn into_header_value(self) -> HeaderValue {
        let value: EncodingHeaderValue = self.into();
        value.into_header_value()
    }
}

//
// EncodingHeaderValue
//

/// Encoding header value.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum EncodingHeaderValue {
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

impl Parseable for EncodingHeaderValue {
    fn parse(representation: &str) -> Option<Self> {
        if representation.eq_ignore_ascii_case("identity") {
            Some(Self::Identity)
        } else if representation.eq_ignore_ascii_case("br") {
            Some(Self::Brotli)
        } else if representation.eq_ignore_ascii_case("deflate") {
            Some(Self::Deflate)
        } else if representation.eq_ignore_ascii_case("gzip") {
            Some(Self::GZip)
        } else if representation.eq_ignore_ascii_case("zstd") {
            Some(Self::Zstandard)
        } else {
            None
        }
    }
}

impl IntoHeaderValue for EncodingHeaderValue {
    fn into_header_value(self) -> HeaderValue {
        HeaderValue::from_static(match self {
            Self::Identity => "identity",
            Self::Brotli => "br",
            Self::Deflate => "deflate",
            Self::GZip => "gzip",
            Self::Zstandard => "zstd",
        })
    }
}

impl From<Encoding> for EncodingHeaderValue {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Identity => Self::Identity,
            Encoding::Brotli => Self::Brotli,
            Encoding::Deflate => Self::Deflate,
            Encoding::GZip => Self::GZip,
            Encoding::Zstandard => Self::Zstandard,
        }
    }
}

impl Into<Encoding> for EncodingHeaderValue {
    fn into(self) -> Encoding {
        match self {
            Self::Identity => Encoding::Identity,
            Self::Brotli => Encoding::Brotli,
            Self::Deflate => Encoding::Deflate,
            Self::GZip => Encoding::GZip,
            Self::Zstandard => Encoding::Zstandard,
        }
    }
}

impl fmt::Display for EncodingHeaderValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity => "identity",
            Self::Brotli => "br",
            Self::Deflate => "deflate",
            Self::GZip => "gzip",
            Self::Zstandard => "zstd",
        }
        .fmt(formatter)
    }
}
