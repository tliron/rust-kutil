use super::super::{errors::*, headers::*};

use {
    async_compression::tokio::{bufread, write},
    bytes::*,
    http::header::*,
    std::fmt,
    tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader},
};

//
// Encoding
//

/// Encoding.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Encoding {
    /// Identity.
    Identity,

    /// Brotli.
    Brotli,

    /// Deflate.
    Deflate,

    /// GZip.
    GZip,

    /// Zstd.
    Zstd,
}

impl Encoding {
    /// Parse.
    pub fn parse(representation: &str) -> Option<Self> {
        if representation.eq_ignore_ascii_case("identity") {
            Some(Encoding::Identity)
        } else if representation.eq_ignore_ascii_case("br") {
            Some(Encoding::Brotli)
        } else if representation.eq_ignore_ascii_case("deflate") {
            Some(Encoding::Deflate)
        } else if representation.eq_ignore_ascii_case("gzip") {
            Some(Encoding::GZip)
        } else if representation.eq_ignore_ascii_case("zstd") {
            Some(Encoding::Zstd)
        } else {
            None
        }
    }

    /// Parse from headers.
    pub fn parse_from_headers(headers: &HeaderMap) -> Self {
        match headers.get(CONTENT_ENCODING) {
            Some(encoding) => match encoding.to_str() {
                Ok(encoding) => Self::parse(encoding).unwrap_or(Self::Identity),
                Err(_) => Self::Identity,
            },
            None => Self::Identity,
        }
    }

    /// Encode.
    pub async fn encode(&self, identity_bytes: &Bytes) -> Result<Bytes, BoxedError> {
        match self {
            Self::Identity => Ok(identity_bytes.clone()),

            Self::Brotli => {
                let mut encoder = write::BrotliEncoder::new(Vec::new());
                encoder.write_all(identity_bytes.as_ref()).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Self::Deflate => {
                let mut encoder = write::DeflateEncoder::new(Vec::new());
                encoder.write_all(identity_bytes.as_ref()).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Self::GZip => {
                let mut encoder = write::GzipEncoder::new(Vec::new());
                encoder.write_all(identity_bytes.as_ref()).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }

            Self::Zstd => {
                let mut encoder = write::ZstdEncoder::new(Vec::new());
                encoder.write_all(identity_bytes.as_ref()).await?;
                encoder.shutdown().await?;
                Ok(encoder.into_inner().into())
            }
        }
    }

    /// Decode.
    pub async fn decode(&self, bytes: &Bytes) -> Result<Bytes, BoxedError> {
        match self {
            Self::Identity => Ok(bytes.clone()),

            Self::Brotli => {
                let mut decoder = bufread::BrotliDecoder::new(BufReader::new(bytes.as_ref()));
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Self::Deflate => {
                let mut decoder = bufread::DeflateDecoder::new(BufReader::new(bytes.as_ref()));
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Self::GZip => {
                let mut decoder = bufread::GzipDecoder::new(BufReader::new(bytes.as_ref()));
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }

            Self::Zstd => {
                let mut decoder = bufread::ZstdDecoder::new(BufReader::new(bytes.as_ref()));
                let mut buffer = Vec::new();
                decoder.read_to_end(&mut buffer).await?;
                Ok(buffer.into())
            }
        }
    }
}

impl Into<HeaderValue> for &Encoding {
    fn into(self) -> HeaderValue {
        match self {
            Encoding::Identity => HeaderValue::from_static("identity"),
            Encoding::Brotli => HeaderValue::from_static("br"),
            Encoding::Deflate => HeaderValue::from_static("deflate"),
            Encoding::GZip => HeaderValue::from_static("gzip"),
            Encoding::Zstd => HeaderValue::from_static("zstd"),
        }
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

//
// WeightedEncoding
//

/// Weighted encoding.
pub struct WeightedEncoding {
    /// Encoding.
    pub encoding: Encoding,

    /// Weight.
    pub weight: QValue,
}
