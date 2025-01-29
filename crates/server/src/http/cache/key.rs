use {
    http::{Method, Request, Uri},
    std::fmt,
};

//
// CacheKey
//

/// Cache key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CacheKey {
    method: Method,
    uri: Uri,
}

impl CacheKey {
    /// Constructor.
    pub fn new(method: Method, uri: Uri) -> Self {
        Self { method, uri }
    }

    /// Constructor.
    pub fn new_from<RequestBodyT>(request: &Request<RequestBodyT>) -> Self {
        Self::new(request.method().clone(), request.uri().clone())
    }
}

impl fmt::Display for CacheKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.method, self.uri)
    }
}
