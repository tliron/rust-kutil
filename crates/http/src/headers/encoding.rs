use super::into::*;

use {
    http::header::*,
    kutil_std::string::*,
    kutil_transcoding::*,
    std::{convert::*, fmt, str::*},
};

impl IntoHeaderValue for Encoding {
    fn into_header_value(self) -> HeaderValue {
        let value: EncodingHeaderValue = self.into();
        value.into()
    }
}

//
// EncodingHeaderValue
//

/// [Encoding] header value.
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

impl Into<HeaderValue> for EncodingHeaderValue {
    fn into(self) -> HeaderValue {
        HeaderValue::from_static(match self {
            Self::Identity => "identity",
            Self::Brotli => "br",
            Self::Deflate => "deflate",
            Self::GZip => "gzip",
            Self::Zstandard => "zstd",
        })
    }
}

impl FromStr for EncodingHeaderValue {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        if representation.eq_ignore_ascii_case("identity") {
            Ok(Self::Identity)
        } else if representation.eq_ignore_ascii_case("br") {
            Ok(Self::Brotli)
        } else if representation.eq_ignore_ascii_case("deflate") {
            Ok(Self::Deflate)
        } else if representation.eq_ignore_ascii_case("gzip") {
            Ok(Self::GZip)
        } else if representation.eq_ignore_ascii_case("zstd") {
            Ok(Self::Zstandard)
        } else {
            Err(format!("unsupported: {}", representation).into())
        }
    }
}

impl fmt::Display for EncodingHeaderValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::Identity => "identity",
                Self::Brotli => "br",
                Self::Deflate => "deflate",
                Self::GZip => "gzip",
                Self::Zstandard => "zstd",
            },
            formatter,
        )
    }
}
