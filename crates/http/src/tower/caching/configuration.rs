use super::{
    super::super::{cache::*, headers::*},
    hooks::*,
};

/// Encodings in order from most preferred to least.
///
/// We are generally preferring to optimize for compute rather than bandwidth.
///
/// GZip and Deflate are almost identical, but we prefer GZip because it allows clients to check
/// for errors.
pub const ENCODINGS_BY_PREFERENCE: &[EncodingHeaderValue] = &[
    EncodingHeaderValue::Brotli,
    EncodingHeaderValue::GZip,
    EncodingHeaderValue::Deflate,
    EncodingHeaderValue::Zstandard,
];

//
// CachingConfiguration
//

/// Caching configuration.
#[derive(Clone)]
pub struct CachingConfiguration<CacheT, CacheKeyT> {
    /// Cache.
    pub cache: Option<CacheT>,

    /// Minimum body size.
    pub min_body_size: usize,

    /// Maximum body size.
    pub max_body_size: usize,

    /// Cacheable by default.
    pub cacheable_by_default: bool,

    /// Cacheable by request (hook).
    pub cacheable_by_request: Option<CacheableHook>,

    /// Cacheable by response (hook).
    pub cacheable_by_response: Option<CacheableHook>,

    /// Cache key (hook).
    pub cache_key: Option<CacheKeyHook<CacheKeyT>>,

    /// Cache duration (hook).
    pub cache_duration: Option<CacheDurationHook>,
}

impl<CacheT, CacheKeyT> Default for CachingConfiguration<CacheT, CacheKeyT> {
    fn default() -> Self {
        Self {
            cache: None,
            min_body_size: 0,
            max_body_size: 1024 * 1024, // 1 MiB
            cacheable_by_default: true,
            cacheable_by_request: None,
            cacheable_by_response: None,
            cache_key: None,
            cache_duration: None,
        }
    }
}

//
// EncodingConfiguration
//

/// Encoding configuration.
#[derive(Clone)]
pub struct EncodingConfiguration {
    /// Enabled encodings in order of preference.
    pub enabled_encodings_by_preference: Option<Vec<EncodingHeaderValue>>,

    /// Encodable by default.
    pub encodable_by_default: bool,

    /// Encodable by request (hook).
    pub encodable_by_request: Option<EncodableHook>,

    /// Encodable by response (hook).
    pub encodable_by_response: Option<EncodableHook>,

    /// Keep identity encoding.
    pub keep_identity_encoding: bool,
}

impl Default for EncodingConfiguration {
    fn default() -> Self {
        Self {
            enabled_encodings_by_preference: Some(ENCODINGS_BY_PREFERENCE.into()),
            encodable_by_default: true,
            encodable_by_request: None,
            encodable_by_response: None,
            keep_identity_encoding: true,
        }
    }
}
