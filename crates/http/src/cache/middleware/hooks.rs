use {http::*, kutil_transcoding::*, std::sync::*};

/// Hook to check if a request or a response is cacheable.
pub type CacheableHook = Arc<Box<dyn Fn(CacheableHookContext) -> bool + Send + Sync>>;

/// Hook to check if a request or a response is encodable.
pub type EncodableHook = Arc<Box<dyn Fn(EncodableHookContext) -> bool + Send + Sync>>;

/// Hook to update a request's cache key.
pub type CacheKeyHook<CacheKeyT> = Arc<Box<dyn Fn(CacheKeyHookContext<CacheKeyT>) + Send + Sync>>;

//
// CacheableHookContext
//

/// Context for [CacheableHook].
pub struct CacheableHookContext<'own> {
    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own> CacheableHookContext<'own> {
    /// Constructor.
    pub fn new(uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { uri, headers }
    }
}

//
// EncodableHookContext
//

/// Context for [EncodableHook].
pub struct EncodableHookContext<'own> {
    /// Encoding.
    pub encoding: &'own Encoding,

    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own> EncodableHookContext<'own> {
    /// Constructor.
    pub fn new(encoding: &'own Encoding, uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { encoding, uri, headers }
    }
}

//
// CacheKeyHookContext
//

/// Context for [CacheKeyHook].
pub struct CacheKeyHookContext<'own, CacheKeyT> {
    /// Cache key.
    pub cache_key: &'own mut CacheKeyT,

    /// Method.
    pub method: &'own Method,

    /// URI.
    pub uri: &'own Uri,

    /// Headers.
    pub headers: &'own HeaderMap,
}

impl<'own, CacheKeyT> CacheKeyHookContext<'own, CacheKeyT> {
    /// Constructor.
    pub fn new(cache_key: &'own mut CacheKeyT, method: &'own Method, uri: &'own Uri, headers: &'own HeaderMap) -> Self {
        Self { cache_key, method, uri, headers }
    }
}
