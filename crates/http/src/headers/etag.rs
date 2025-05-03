use super::{super::cache::*, parseable::*, preferences::*};

use std::fmt;

//
// ETag
//

/// ETag.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ETag {
    /// Tag.
    pub tag: String,

    /// Weak.
    pub weak: bool,
}

impl ETag {
    /// Constructor.
    pub fn new(tag: String, weak: bool) -> Self {
        Self { tag, weak }
    }

    /// Parse list.
    pub fn parse_list(representation: &str) -> Option<Vec<Self>> {
        let languages: Vec<_> = representation.split(",").map(|etag| ETag::parse(etag)).flatten().collect();
        if !languages.is_empty() { Some(languages) } else { None }
    }
}

impl Parseable for ETag {
    fn parse(representation: &str) -> Option<Self> {
        let mut tag = representation.trim();

        if tag.ends_with("\"") {
            tag = &tag[..tag.len() - 1];
        } else {
            return None;
        }

        let weak = if tag.starts_with("W/\"") {
            tag = &tag[3..];
            true
        } else if tag.starts_with("\"") {
            tag = &tag[1..];
            false
        } else {
            return None;
        };

        if tag.contains("\"") {
            return None;
        }

        Some(Self::new(tag.into(), weak))
    }
}

impl Sized for ETag {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<ETag>();
        SELF_SIZE + self.tag.len()
    }
}

impl fmt::Display for ETag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.weak { write!(formatter, "W/\"{}\"", self.tag) } else { write!(formatter, "\"{}\"", self.tag) }
    }
}

//
// ETagList
//

/// List of [ETag].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ETagList(pub Vec<ETag>);

impl Parseable for ETagList {
    fn parse(representation: &str) -> Option<Self> {
        let tags: Vec<_> = representation.split(",").map(|t| ETag::parse(t)).flatten().collect();
        if !tags.is_empty() { Some(Self(tags)) } else { None }
    }
}

//
// ETagMatcher
//

/// ETag matcher.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ETagMatcher(pub Selector<ETagList>);

impl ETagMatcher {
    /// Whether any one of our tags matches the reference.
    ///
    /// [Any](ETagMatch::Any) will always match. Weak tags will *never* match.
    pub fn matches(&self, reference: Option<&ETag>) -> bool {
        return match &self.0 {
            Selector::Any => true,

            Selector::Specific(selector) => {
                if let Some(reference) = reference {
                    if !reference.weak && selector.0.contains(&reference) {
                        return true;
                    }
                }

                false
            }
        };
    }
}

impl Parseable for ETagMatcher {
    fn parse(representation: &str) -> Option<Self> {
        Some(Self(Selector::<ETagList>::parse(representation)?))
    }
}
