use super::super::super::{cache::*, encoding::*, headers::*};

use {
    bytes::*,
    http::{header::*, request::*, response::*},
    http_body::*,
    std::{convert::*, error::Error, pin::*, task::*},
    tower::*,
};

//
// EncodedCachingService
//

/// HTTP encoded caching service.
///
/// Handles both caching and encoding, supporting the standard Brotli, Deflate, GZip, and Zstd encodings.
#[derive(Clone)]
pub struct EncodedCachingService<InnerServiceT> {
    inner_service: InnerServiceT,
    cache: CacheRef,
    encode: bool,
    min_size: usize,
    max_size: usize,
    keep_identity: bool,
}

impl<InnerServiceT> EncodedCachingService<InnerServiceT> {
    /// Constuctor.
    pub fn new(
        inner_service: InnerServiceT,
        cache: CacheRef,
        encode: bool,
        min_size: usize,
        max_size: usize,
        keep_identity: bool,
    ) -> Self {
        Self { inner_service, cache, encode, min_size, max_size, keep_identity }
    }
}

type CapturedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

impl<InnerServiceT, RequestBodyT, ResponseBodyT, ErrorT> Service<Request<RequestBodyT>>
    for EncodedCachingService<InnerServiceT>
where
    InnerServiceT: Service<Request<RequestBodyT>, Response = Response<ResponseBodyT>, Error = ErrorT>,
    InnerServiceT::Future: Send + 'static,
    ResponseBodyT: Body + From<Bytes> + Send + 'static,
    ResponseBodyT::Data: Send,
    ResponseBodyT::Error: Error + Send + Sync,
{
    type Response = InnerServiceT::Response;
    type Error = InnerServiceT::Error;
    type Future = CapturedFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner_service.poll_ready(context)
    }

    fn call(&mut self, request: Request<RequestBodyT>) -> Self::Future {
        // TODO: when skipping must encode!!!

        // Don't use the cache with non-idempotent requests (e.g. POST)
        let method = request.method();
        if !method.is_idempotent() {
            tracing::info!("skip (non-idempotent): {}", method);
            let response_result = self.inner_service.call(request);
            return Box::pin(async move { response_result.await });
        }

        let cache = self.cache.clone();
        let encode = self.encode;
        let min_size = self.min_size;
        let max_size = self.max_size;
        let keep_identity = self.keep_identity;

        let key = CacheKey::new_from(&request);
        let request_headers = request.headers().clone();
        let response_result = self.inner_service.call(request);

        Box::pin(async move {
            match cache.get(&key) {
                Some(mut entry) => {
                    tracing::info!("hit: {}", key);

                    let encoding =
                        if encode { AcceptedEncodings::new(&request_headers).best() } else { Encoding::Identity };

                    match entry.to_response(&encoding, keep_identity).await {
                        Ok((response, modified)) => {
                            if modified {
                                // Note that another thread may have aleady inserted this key,
                                // but our version is fresher
                                cache.insert(key, entry);
                            }

                            Ok(response)
                        }

                        Err(error) => {
                            tracing::warn!("could not create response: {} {}", key, error);
                            Ok(Response::new(Bytes::new().into()))
                        }
                    }

                    // TODO: conditional! might return 304!
                }

                None => {
                    let encoding =
                        if encode { AcceptedEncodings::new(&request_headers).best() } else { Encoding::Identity };

                    let response = response_result.await?;
                    let response_headers = response.headers();

                    if !get_cache_header(response_headers) {
                        tracing::info!("skip (X-Cache=false): {}", key);
                        return Ok(response);
                    }

                    if !response.status().is_success() {
                        tracing::info!("skip (status {}): {}", response.status().as_u16(), key);
                        return Ok(response);
                    }

                    if response_headers.contains_key(CONTENT_RANGE) {
                        tracing::info!("skip (range): {}", key);
                        return Ok(response);
                    }

                    let content_length = match get_content_length_header(response_headers) {
                        Some(content_length) => {
                            if content_length < min_size {
                                tracing::info!("skip (too small): {}", key);
                                return Ok(response);
                            } else if content_length > max_size {
                                tracing::info!("skip (too big): {}", key);
                                return Ok(response);
                            }
                            content_length
                        }

                        None => {
                            // When Content-Length is not present, we probably have Transfer-Encoding instead
                            tracing::info!("skip (stream): {}", key);
                            return Ok(response);
                        }
                    };

                    tracing::info!("miss: {}", key);

                    match CacheEntry::new(response, encoding.clone(), content_length, keep_identity).await {
                        Ok(mut entry) => {
                            match entry.to_response(&encoding, keep_identity).await {
                                Ok((response, _)) => {
                                    tracing::info!("store: {}", key);

                                    // Note that another thread may have aleady inserted this key,
                                    // but our version is fresher
                                    cache.insert(key, entry);

                                    Ok(response)
                                }

                                Err(error) => {
                                    tracing::warn!("could not create response: {} {}", key, error);
                                    Ok(Response::new(Bytes::new().into()))
                                }
                            }
                        }

                        Err(error) => {
                            tracing::warn!("could not create cache entry: {} {}", key, error);
                            Ok(Response::new(Bytes::new().into()))
                        }
                    }
                }
            }
        })
    }
}
