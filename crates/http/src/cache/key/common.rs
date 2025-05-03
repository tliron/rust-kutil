use super::{
    super::{super::headers::*, sized::*},
    key::*,
};

use {
    http::{header::*, uri::*, *},
    std::{collections::*, fmt, hash::*},
    url::form_urlencoded,
};

//
// CommonCacheKey
//

/// [CacheKey] implementation designed for common use cases.
///
/// Note that query parameters are sorted so that their order would not affect caching.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommonCacheKey {
    /// Method.
    pub method: Method,

    /// Optional scheme.
    ///
    /// This will often *not* be provided by upstream routers.
    pub scheme: Option<Scheme>,

    /// Optional authority.
    ///
    /// This will often *not* be provided by upstream routers.
    pub authority: Option<Authority>,

    /// Optional path.
    pub path: Option<String>,

    /// Optional query (sorted by key).
    pub query: Option<BTreeMap<String, String>>,

    /// Optional media type.
    ///
    /// Not used by our [CacheKey] implementation, but reserved for custom use.
    pub media_type: Option<MediaType>,

    /// Optional languages (sorted).
    ///
    /// Not used by our [CacheKey] implementation, but reserved for custom use.
    pub languages: Option<Vec<Language>>,
}

impl CommonCacheKey {
    /// Constructor.
    pub fn new(
        method: Method,
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        path: Option<String>,
        query: Option<BTreeMap<String, String>>,
        media_type: Option<MediaType>,
        languages: Option<Vec<Language>>,
    ) -> Self {
        Self { method, scheme, authority, path, query, media_type, languages }
    }
}

impl CacheKey for CommonCacheKey {
    fn for_request(method: &Method, uri: &Uri, _headers: &HeaderMap) -> Self {
        let (path, query) = match uri.path_and_query() {
            Some(path_and_query) => (
                Some(path_and_query.path().into()),
                path_and_query.query().map(|query| form_urlencoded::parse(query.as_bytes()).into_owned().collect()),
            ),

            None => (None, None),
        };

        Self::new(method.clone(), uri.scheme().cloned(), uri.authority().cloned(), path, query, None, None)
    }
}

impl Sized for CommonCacheKey {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<CommonCacheKey>();

        let mut size = SELF_SIZE;

        if let Some(authority) = &self.authority {
            size += authority.as_str().len();
        }

        if let Some(path) = &self.path {
            size += path.as_str().len();
        }

        if let Some(query) = &self.query {
            for (k, v) in query {
                size += k.len() + v.len();
            }
        }

        if let Some(media_type) = &self.media_type {
            size += media_type.size();
        }

        if let Some(languages) = &self.languages {
            for language in languages {
                size += language.size();
            }
        }
        size
    }
}

impl fmt::Display for CommonCacheKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scheme = self.scheme.as_ref().map(|s| s.as_str()).unwrap_or_default();
        let authority = self.authority.as_ref().map(|a| a.as_str()).unwrap_or_default();
        let path = self.path.as_ref().map(|p| p.as_str()).unwrap_or_default();

        let query = self
            .query
            .as_ref()
            .map(|parameter| {
                let mut string = String::new();
                for (k, v) in parameter {
                    if !string.is_empty() {
                        string += "&"
                    }
                    string += &(String::from(k) + "=" + v);
                }
                string
            })
            .unwrap_or_default();

        let media_type = self.media_type.as_ref().map(|m| m.to_string()).unwrap_or_default();

        let languages = self
            .languages
            .as_ref()
            .map(|languages| {
                let languages: Vec<_> = languages.iter().map(|language| language.to_string()).collect();
                languages.join(",")
            })
            .unwrap_or_default();

        write!(formatter, "{}|{}|{}|{}|{}|{}|{}", self.method, scheme, authority, path, query, media_type, languages)
    }
}
