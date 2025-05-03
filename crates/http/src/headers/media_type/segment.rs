use super::super::super::cache::*;

use {
    kutil_std::{foster::*, *},
    std::{convert::*, str::*},
};

//
// MediaTypeSegment
//

/// [MediaType](super::media_type::MediaType) segment.
#[derive(Clone, Debug)]
pub struct MediaTypeSegment(pub FosterByteStr);

delegate_newtype_of_foster_bytestr!(MediaTypeSegment);

impl CacheWeight for MediaTypeSegment {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSegment>();
        SELF_SIZE + self.0.len()
    }
}

impl FromStr for MediaTypeSegment {
    type Err = Infallible;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(representation.into())
    }
}
