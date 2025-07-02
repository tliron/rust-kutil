use super::into::*;

use {http::header::*, kutil_std::*, kutil_transcoding::*, std::convert::*};

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
#[derive(Clone, Copy, Debug, Default, Display, FromStr, Eq, Hash, PartialEq)]
#[display(lowercase)]
#[from_str(lowercase)]
pub enum EncodingHeaderValue {
    /// Identity.
    #[default]
    Identity,

    /// Brotli.
    #[strings("br")]
    Brotli,

    /// Deflate.
    Deflate,

    /// GZip.
    GZip,

    /// Zstandard.
    #[strings("zstd")]
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
        HeaderValue::from_static(self.into())
    }
}
