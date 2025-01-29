use super::{
    super::{body::*, encoding::*, errors::*},
    headers::*,
};

use {
    bytes::*,
    http::{HeaderMap, Response, StatusCode, header::*},
    http_body::*,
    std::collections::*,
};

//
// CacheEntry
//

/// Cached HTTP response.
///
/// Can contain one or more encodings of the same body.
#[derive(Clone, Debug)]
pub struct CacheEntry {
    status: StatusCode,
    headers: HeaderMap,
    body: HashMap<Encoding, Bytes>,
}

impl CacheEntry {
    /// Constructor.
    pub async fn new<ResponseBodyT>(
        response: Response<ResponseBodyT>,
        as_encoding: Encoding,
        max_size: usize,
        store_identity: bool,
    ) -> Result<Self, BoxedError>
    where
        ResponseBodyT: Body,
        ResponseBodyT::Error: Into<BoxedError>,
    {
        let status = response.status();

        let mut headers = response.headers().clone();
        let response_encoding = Encoding::parse_from_headers(&headers);

        headers.remove(CACHE_HEADER);
        headers.remove(CONTENT_ENCODING);
        headers.remove(CONTENT_LENGTH);

        let bytes = response.into_body().to_bytes(max_size).await?;

        let mut body = HashMap::new();

        if as_encoding == response_encoding {
            // It's already in the right encoding
            body.insert(as_encoding, bytes);
        } else if response_encoding == Encoding::Identity {
            // Encode from identity
            tracing::info!("encoding to {}", as_encoding);
            body.insert(as_encoding.clone(), as_encoding.encode(&bytes).await?);
            if store_identity {
                body.insert(Encoding::Identity, bytes);
            }
        } else if as_encoding == Encoding::Identity {
            // Decode into identity
            tracing::info!("decoding from {}", response_encoding);
            let identity_bytes = response_encoding.decode(&bytes).await?;
            body.insert(as_encoding, identity_bytes);
        } else {
            // Reencode (via identity)
            tracing::info!("reencoding from {} to {}", response_encoding, as_encoding);
            let identity_bytes = response_encoding.decode(&bytes).await?;
            body.insert(as_encoding.clone(), as_encoding.encode(&identity_bytes).await?);
            if store_identity {
                body.insert(Encoding::Identity, identity_bytes);
            }
        }

        Ok(Self { status, headers, body })
    }

    /// To response.
    ///
    /// Will contain a clone of the bytes.
    ///
    /// If we don't have the required encoding then we will reencode from another encoding,
    /// storing it for future use. This can cause the identity encoding to be stored.
    ///
    /// Returns true if modified, meaning that a new encoding was stored.
    pub async fn to_response<ResponseBodyT>(
        &mut self,
        encoding: &Encoding,
        store_identity: bool,
    ) -> Result<(Response<ResponseBodyT>, bool), BoxedError>
    where
        ResponseBodyT: Body + From<Bytes>,
    {
        let (bytes, modified) = self.bytes(encoding, store_identity).await?;
        let length = bytes.len();
        let mut response = Response::new(bytes.into());

        *response.status_mut() = self.status;

        *response.headers_mut() = self.headers.clone();
        if *encoding != Encoding::Identity {
            response.headers_mut().insert(CONTENT_ENCODING, encoding.into());
        }
        response.headers_mut().insert(CONTENT_LENGTH, length.into());

        Ok((response, modified))
    }

    /// Returns a clone of the bytes in the required encoding.
    ///
    /// If we don't have the required encoding then we will reencode from another encoding,
    /// storing it for future use. This can cause the identity encoding to be stored.
    ///
    /// Returns true if modified, meaning that a new encoding was stored.
    pub async fn bytes(&mut self, encoding: &Encoding, store_identity: bool) -> Result<(Bytes, bool), BoxedError> {
        match self.body.get(encoding) {
            Some(bytes) => Ok((bytes.clone(), false)),

            None => match encoding {
                Encoding::Identity => {
                    // Try stored encodings in order from cheapest to decompress to most expensive
                    if let Some(zstd_bytes) = self.body.get(&Encoding::Zstd) {
                        tracing::info!("decoding from Zstd");
                        let identity_bytes = Encoding::Zstd.decode(zstd_bytes).await?;
                        self.body.insert(Encoding::Identity, identity_bytes.clone());
                        Ok((identity_bytes, true))
                    } else if let Some(deflate_bytes) = self.body.get(&Encoding::Deflate) {
                        tracing::info!("decoding from Deflate");
                        let identity_bytes = Encoding::Deflate.decode(deflate_bytes).await?;
                        self.body.insert(Encoding::Identity, identity_bytes.clone());
                        Ok((identity_bytes, true))
                    } else if let Some(gzip_bytes) = self.body.get(&Encoding::GZip) {
                        tracing::info!("decoding from GZip");
                        let identity_bytes = Encoding::GZip.decode(gzip_bytes).await?;
                        self.body.insert(Encoding::Identity, identity_bytes.clone());
                        Ok((identity_bytes, true))
                    } else if let Some(brotli_bytes) = self.body.get(&Encoding::Brotli) {
                        tracing::info!("decoding from Brotli");
                        let identity_bytes = Encoding::Brotli.decode(brotli_bytes).await?;
                        self.body.insert(Encoding::Identity, identity_bytes.clone());
                        Ok((identity_bytes, true))
                    } else {
                        // This should never happen (but we don't want to panic here!)
                        tracing::error!("CacheEntry without any encodings");
                        Ok((Bytes::new(), false))
                    }
                }

                _ => {
                    // Try stored encodings in order from cheapest to decompress to most expensive
                    if let Some(identity_bytes) = self.body.get(&Encoding::Identity) {
                        tracing::info!("encoding to {}", encoding);
                        let bytes = encoding.encode(identity_bytes).await?;
                        self.body.insert(encoding.clone(), bytes.clone());
                        Ok((bytes, true))
                    } else if let Some(zstd_bytes) = self.body.get(&Encoding::Zstd) {
                        tracing::info!("reencoding from Zstd to {}", encoding);
                        let identity_bytes = Encoding::Zstd.decode(zstd_bytes).await?;
                        let bytes = encoding.encode(&identity_bytes).await?;
                        if store_identity {
                            self.body.insert(Encoding::Identity, identity_bytes);
                        }
                        self.body.insert(encoding.clone(), bytes.clone());
                        Ok((bytes, true))
                    } else if let Some(deflate_bytes) = self.body.get(&Encoding::Deflate) {
                        tracing::info!("reencoding from Deflate to {}", encoding);
                        let identity_bytes = Encoding::Deflate.decode(deflate_bytes).await?;
                        let bytes = encoding.encode(&identity_bytes).await?;
                        if store_identity {
                            self.body.insert(Encoding::Identity, identity_bytes);
                        }
                        self.body.insert(encoding.clone(), bytes.clone());
                        Ok((bytes, true))
                    } else if let Some(gzip_bytes) = self.body.get(&Encoding::GZip) {
                        tracing::info!("reencoding from GZip to {}", encoding);
                        let identity_bytes = Encoding::GZip.decode(gzip_bytes).await?;
                        let bytes = encoding.encode(&identity_bytes).await?;
                        if store_identity {
                            self.body.insert(Encoding::Identity, identity_bytes);
                        }
                        self.body.insert(encoding.clone(), bytes.clone());
                        Ok((bytes, true))
                    } else if let Some(brotli_bytes) = self.body.get(&Encoding::Brotli) {
                        tracing::info!("reencoding from Brotli to {}", encoding);
                        let identity_bytes = Encoding::Brotli.decode(brotli_bytes).await?;
                        let bytes = encoding.encode(&identity_bytes).await?;
                        if store_identity {
                            self.body.insert(Encoding::Identity, identity_bytes);
                        }
                        self.body.insert(encoding.clone(), bytes.clone());
                        Ok((bytes, true))
                    } else {
                        // This should never happen (but we don't want to panic here!)
                        tracing::error!("CacheEntry without any encodings");
                        Ok((Bytes::new(), false))
                    }
                }
            },
        }
    }
}
