use super::headers::*;

use {http::*, std::time::*};

/// `XX-Cache` HTTP response header specifying whether to cache the response.
pub const CACHE_HEADER: HeaderName = HeaderName::from_static("xx-cache");

/// `XX-Cache-Duration` HTTP response header specifying the cache duration in seconds.
pub const CACHE_DURATION_HEADER: HeaderName = HeaderName::from_static("xx-cache-duration");

/// `XX-Encode` HTTP response header specifying whether to encode the response.
pub const ENCODE_HEADER: HeaderName = HeaderName::from_static("xx-encode");

/// `Content-Digest` HTTP response header.
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Digest).
///
/// (For some reason missing in the [http](https://github.com/hyperium/http) library.)
pub const CONTENT_DIGEST: HeaderName = HeaderName::from_static("content-digest");

//
// ParseCustomHeaders
//

/// Parse custom headers.
pub trait ParseCustomHeaders {
    /// Parse `XX-Cache` response header value.
    fn cache(&self, default: bool) -> bool;

    /// Parse `XX-Cache-Duration` response header value.
    fn cache_duration(&self) -> Option<Duration>;

    /// Parse `XX-Encode` response header value.
    fn encode(&self, default: bool) -> bool;
}

impl ParseCustomHeaders for HeaderMap {
    fn cache(&self, default: bool) -> bool {
        self.bool_value(CACHE_HEADER, default)
    }

    fn cache_duration(&self) -> Option<Duration> {
        self.duration_value(CACHE_DURATION_HEADER)
    }

    fn encode(&self, default: bool) -> bool {
        self.bool_value(ENCODE_HEADER, default)
    }
}
