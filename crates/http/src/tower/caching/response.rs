use super::{
    super::super::{cache::*, headers::*, transcoding::*},
    hooks::*,
};

use {
    ::bytes::*,
    http::{header::*, *},
    http_body::*,
    kutil_transcoding::*,
    std::{error::Error, sync::*},
};

//
// ToTranscodingResponse
//

/// To transcoding response.
#[allow(async_fn_in_trait)]
pub trait ToTranscodingResponse {
    /// To a [Response] with a [TranscodingBody].
    ///
    /// Will update the cache if we are modified.
    ///
    /// If we encounter an error will return a response with [StatusCode::INTERNAL_SERVER_ERROR].
    async fn to_transcoding_response<ResponseBodyT, CacheT, CacheKeyT>(
        self,
        is_new: bool,
        key: CacheKeyT,
        cache: CacheT,
        encoding: &Encoding,
        encodable_by_default: bool,
        keep_identity_encoding: bool,
    ) -> Response<TranscodingBody<ResponseBodyT>>
    where
        ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
        ResponseBodyT::Data: From<Bytes> + Send,
        ResponseBodyT::Error: Error + Send + Sync,
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey;
}

impl ToTranscodingResponse for CachedResponseRef {
    /// To a [Response] with a [TranscodingBody].
    ///
    /// Will update the cache if we are modified.
    ///
    /// If we encounter an error will return a response with [StatusCode::INTERNAL_SERVER_ERROR].
    async fn to_transcoding_response<ResponseBodyT, CacheT, CacheKeyT>(
        self,
        is_new: bool,
        key: CacheKeyT,
        cache: CacheT,
        encoding: &Encoding,
        encodable_by_default: bool,
        keep_identity_encoding: bool,
    ) -> Response<TranscodingBody<ResponseBodyT>>
    where
        ResponseBodyT: 'static + Body + From<Bytes> + Send + Unpin,
        ResponseBodyT::Data: From<Bytes> + Send,
        ResponseBodyT::Error: Error + Send + Sync,
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey,
    {
        match self.to_response(&encoding, encodable_by_default, keep_identity_encoding).await {
            Ok((response, modified)) => {
                if is_new {
                    cache.put(key, self).await;
                } else if let Some(modified) = modified {
                    // A new CachedResponse should already contain our encoding
                    // and thus never cause modification!
                    assert!(!is_new);

                    cache.put(key, Arc::new(modified)).await;
                }

                response
            }

            Err(error) => {
                tracing::error!("could not create response from cache: {} {}", key, error);
                error_transcoding_response()
            }
        }
    }
}

//
// UpstreamResponse
//

/// Upstream response.
pub trait UpstreamResponse<ResponseBodyT> {
    /// Check if we should skip the cache.
    ///
    /// If the response passes all our checks then we turn to the hook to give it one last chance
    /// to skip the cache.
    fn should_skip_cache(
        &self,
        uri: &Uri,
        min_body_size: usize,
        max_body_size: usize,
        cacheable_by_default: bool,
        cacheable_by_response: Option<CacheableHook>,
    ) -> (bool, Option<usize>);

    /// Validate encoding.
    ///
    /// Gives the hook one last chance to skip encoding. Will return true if the hook forced a skip.
    fn validate_encoding(
        &self,
        encoding: Encoding,
        uri: &Uri,
        encodable_by_response: Option<EncodableHook>,
    ) -> (Encoding, bool);
}

impl<ResponseBodyT> UpstreamResponse<ResponseBodyT> for Response<ResponseBodyT> {
    // Also returns `Content-Length` if available.
    fn should_skip_cache(
        &self,
        uri: &Uri,
        min_body_size: usize,
        max_body_size: usize,
        cacheable_by_default: bool,
        cacheable_by_response: Option<CacheableHook>,
    ) -> (bool, Option<usize>) {
        let headers = self.headers();
        let status = self.status();

        let mut skip_cache = if !headers.cache(cacheable_by_default) {
            tracing::debug!("skip (XX-Cache=false)");
            (true, None)
        } else if !status.is_success() {
            tracing::debug!("skip (status={})", status.as_u16());
            (true, None)
        } else if headers.contains_key(CONTENT_RANGE) {
            tracing::debug!("skip (range)");
            (true, None)
        } else {
            match headers.content_length() {
                Some(content_length) => {
                    if content_length < min_body_size {
                        tracing::debug!("skip (Content-Length too small)");
                        (true, None)
                    } else if content_length > max_body_size {
                        tracing::debug!("skip (Content-Length too big)");
                        (true, None)
                    } else {
                        (false, Some(content_length))
                    }
                }

                None => (false, None),
            }
        };

        if !skip_cache.0 {
            if let Some(cacheable) = cacheable_by_response {
                if !cacheable(CacheableHookContext::new(uri, headers)) {
                    tracing::debug!("skip (cacheable_by_response=false)");
                    skip_cache.0 = true;
                }
            }
        }

        skip_cache
    }

    fn validate_encoding(
        &self,
        encoding: Encoding,
        uri: &Uri,
        encodable_by_response: Option<EncodableHook>,
    ) -> (Encoding, bool) {
        if encoding == Encoding::Identity {
            (encoding, false)
        } else {
            match encodable_by_response {
                Some(encodable) => {
                    if encodable(EncodableHookContext::new(&encoding, uri, self.headers())) {
                        (encoding, false)
                    } else {
                        tracing::debug!("not encoding to {} (encodable_by_response=false)", encoding);
                        (Encoding::Identity, true)
                    }
                }

                None => (encoding, false),
            }
        }
    }
}
