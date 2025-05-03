use super::super::{super::cache::*, parseable::*};

use kutil_std::{foster::*, foster_string_newtype};

//
// MediaTypeSegment
//

/// [MediaType] segment.
#[derive(Clone, Debug)]
pub struct MediaTypeSegment(pub FosterString);

foster_string_newtype!(MediaTypeSegment);

impl Parseable for MediaTypeSegment {
    fn parse(representation: &str) -> Option<Self> {
        Some(representation.into())
    }
}

impl Sized for MediaTypeSegment {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSegment>();
        SELF_SIZE + self.0.len()
    }
}
