use super::{
    super::{super::headers::*, weight::*},
    key::*,
};

use {
    bytes::*,
    bytestr::*,
    http::{header::*, uri::*, *},
    std::{collections::*, fmt, hash::*},
    url::form_urlencoded,
};

//
// CommonCacheKey
//

/// [CacheKey] implementation designed for common use cases.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommonCacheKey {
    /// Method.
    pub method: Method,

    /// Optional authority.
    ///
    /// This will often *not* be provided by upstream requests.
    pub authority: Option<Authority>,

    /// Optional scheme.
    ///
    /// This will often *not* be provided by upstream requests.
    pub scheme: Option<Scheme>,

    /// Optional host.
    ///
    /// This will often *not* be provided by upstream requests.
    pub host: Option<ByteStr>,

    /// Optional port.
    ///
    /// This will often *not* be provided by upstream requests.
    pub port: Option<u16>,

    /// Optional path.
    pub path: Option<ByteStr>,

    /// Optional query (sorted by key).
    pub query: Option<BTreeMap<ByteStr, ByteStr>>,

    /// Optional media type.
    ///
    /// Not set by default but reserved for custom use.
    pub media_type: Option<MediaType>,

    /// Optional languages (sorted).
    ///
    /// Not set by default but reserved for custom use.
    pub languages: Option<Vec<Language>>,

    /// Optional extensions (sorted by key).
    ///
    /// Not set by default but reserved for custom use.
    pub extensions: Option<BTreeMap<Bytes, Bytes>>,
}

impl CommonCacheKey {
    /// Constructor.
    pub fn new(
        method: Method,
        scheme: Option<Scheme>,
        authority: Option<Authority>,
        host: Option<ByteStr>,
        port: Option<u16>,
        path: Option<ByteStr>,
        query: Option<BTreeMap<ByteStr, ByteStr>>,
        media_type: Option<MediaType>,
        languages: Option<Vec<Language>>,
        extensions: Option<BTreeMap<Bytes, Bytes>>,
    ) -> Self {
        Self { method, scheme, authority, host, port, path, query, media_type, languages, extensions }
    }
}

impl CacheKey for CommonCacheKey {
    fn for_request(method: &Method, uri: &Uri, _headers: &HeaderMap) -> Self {
        let (path, query) = match uri.path_and_query() {
            Some(path_and_query) => (
                Some(path_and_query.path().into()),
                path_and_query.query().map(|query| {
                    form_urlencoded::parse(query.as_bytes()).map(|(name, value)| (name.into(), value.into())).collect()
                }),
            ),

            None => (None, None),
        };

        Self::new(
            method.clone(),
            uri.scheme().cloned(),
            uri.authority().cloned(),
            uri.host().map(|host| host.into()),
            uri.port_u16(),
            path,
            query,
            None,
            None,
            None,
        )
    }
}

impl CacheWeight for CommonCacheKey {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<CommonCacheKey>();

        let mut size = SELF_SIZE;

        if let Some(authority) = &self.authority {
            size += authority.as_str().len();
        }

        if let Some(host) = &self.host {
            size += host.len();
        }

        if let Some(path) = &self.path {
            size += path.len();
        }

        if let Some(query) = &self.query {
            for (k, v) in query {
                size += k.len() + v.len();
            }
        }

        if let Some(media_type) = &self.media_type {
            size += media_type.cache_weight();
        }

        if let Some(languages) = &self.languages {
            for language in languages {
                size += language.cache_weight();
            }
        }

        if let Some(extensions) = &self.extensions {
            for (k, v) in extensions {
                size += k.len() + v.len();
            }
        }

        size
    }
}

impl fmt::Display for CommonCacheKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scheme = self.scheme.as_ref().map(|scheme| scheme.as_str()).unwrap_or_default();
        let authority = self.authority.as_ref().map(|authority| authority.as_str()).unwrap_or_default();
        let host = self.host.as_ref().map(|host| host.as_str()).unwrap_or_default();
        let port = self.port.map(|port| port.to_string()).unwrap_or_default();
        let path = self.path.as_ref().map(|path| path.as_str()).unwrap_or_default();

        let query = self
            .query
            .as_ref()
            .map(|parameter| {
                let mut string = String::new();
                for (k, v) in parameter {
                    if !string.is_empty() {
                        string += "&"
                    }
                    string += &format!("{}={}", k, v);
                }
                string
            })
            .unwrap_or_default();

        let media_type = self.media_type.as_ref().map(|media_type| media_type.to_string()).unwrap_or_default();

        let languages = self
            .languages
            .as_ref()
            .map(|languages| {
                let languages: Vec<_> = languages.iter().map(|language| language.to_string()).collect();
                languages.join(",")
            })
            .unwrap_or_default();

        let extensions = self
            .extensions
            .as_ref()
            .map(|extension| {
                let mut string = String::new();
                for (k, v) in extension {
                    if !string.is_empty() {
                        string += "&"
                    }
                    // We only display the length
                    string += &format!("{}={}", k.len(), v.len());
                }
                string
            })
            .unwrap_or_default();

        write!(
            formatter,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.method, scheme, authority, host, port, path, query, media_type, languages, extensions
        )
    }
}
