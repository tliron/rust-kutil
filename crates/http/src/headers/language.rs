use super::{super::cache::*, into::*, parseable::*};

use {
    http::header::*,
    kutil_std::{collections::*, foster::*, foster_string_vector_newtype},
    std::fmt,
};

//
// Language
//

/// Language tag value in HTTP headers.
///
/// See [IETF RFC 5646 section 2.1](https://datatracker.ietf.org/doc/html/rfc5646#section-2.1).
///
/// Stored as a sequence of subtags.
///
/// Note that even though ISO recommends cased representations, they are case-insensitive in HTTP.
/// Thus we convert all subtags to lowercase for comparison efficiency.
#[derive(Clone, Debug)]
pub struct Language(pub FosterStringVector);

foster_string_vector_newtype!(Language);

impl Language {
    /// Parse list.
    pub fn parse_list(representation: &str) -> Option<FastHashSet<Self>> {
        let languages: FastHashSet<_> = representation.split(",").map(|language| language.trim().into()).collect();
        if !languages.is_empty() { Some(languages) } else { None }
    }
}

impl Parseable for Language {
    fn parse(representation: &str) -> Option<Self> {
        Some(representation.into())
    }
}

impl IntoHeaderValue for Language {
    fn into_header_value(self) -> HeaderValue {
        HeaderValue::from_str(&self.to_string()).expect("should be safe")
    }
}

impl Sized for Language {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<Language>();
        let mut size = SELF_SIZE;
        for subtag in &self.0 {
            size += subtag.len();
        }
        size
    }
}

impl fmt::Display for Language {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Foster::Owned(subtags) => subtags.join("-").fmt(formatter),
            Foster::Fostered(subtags) => subtags.join("-").fmt(formatter),
        }
    }
}

impl From<&str> for Language {
    fn from(representation: &str) -> Self {
        Self::new_owned(representation.split("-").map(|subtag| subtag.to_lowercase()).collect())
    }
}
