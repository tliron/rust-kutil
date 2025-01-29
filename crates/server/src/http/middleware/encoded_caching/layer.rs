use super::{super::super::cache::*, service::*};

use tower::Layer;

//
// EncodedCachingLayer
//

/// HTTP encoded caching layer.
///
/// Handles both caching and encoding, supporting the standard Brotli, Deflate, GZip, and Zstd encodings.
#[derive(Clone)]
pub struct EncodedCachingLayer {
    cache: CacheRef,
    encode: bool,
    min_size: usize,
    max_size: usize,
    keep_identity: bool,
}

impl EncodedCachingLayer {
    /// Constructor.
    ///
    /// Defaults to encode true, min_size 0, max_size 1 Mib, and keep_identity true.
    pub fn new(cache: CacheRef) -> Self {
        Self { cache, encode: true, min_size: 0, max_size: 1_048_576, keep_identity: true }
    }

    /// Whether to support encoding of response bodies.
    pub fn encode(mut self, encode: bool) -> Self {
        self.encode = encode;
        self
    }

    /// Minimum size in bytes of the response bodies to cache.
    pub fn min_size(mut self, min_size: usize) -> Self {
        self.min_size = min_size;
        self
    }

    /// Maximum size in bytes of the response bodies to cache.
    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// Whether to keep the identity encoding if created during reencoding.
    ///
    /// Keeping the identity optimizes for compute with the trade-off of using more memory.
    pub fn keep_identity(mut self, keep_identity: bool) -> Self {
        self.keep_identity = keep_identity;
        self
    }
}

impl<InnerT> Layer<InnerT> for EncodedCachingLayer {
    type Service = EncodedCachingService<InnerT>;

    fn layer(&self, inner: InnerT) -> Self::Service {
        EncodedCachingService::new(
            inner,
            self.cache.clone(),
            self.encode,
            self.min_size,
            self.max_size,
            self.keep_identity,
        )
    }
}
