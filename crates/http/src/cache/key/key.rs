use super::super::sized::*;

use {
    http::{header::*, uri::*, *},
    std::{fmt, hash::*},
};

//
// CacheKey
//

/// Cache key.
pub trait CacheKey: 'static + Clone + fmt::Display + Eq + Hash + Send + Sized + Sync {
    /// Create a cache key for a request.
    fn for_request(method: &Method, uri: &Uri, headers: &HeaderMap) -> Self;
}
