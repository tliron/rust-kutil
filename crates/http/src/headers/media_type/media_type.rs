use super::{
    super::{super::cache::*, into::*, parseable::*},
    segment::*,
};

use {
    http::header::*,
    kutil_std::{borrow::*, foster::*},
    std::{cmp::*, fmt, hash::*},
};

//
// MediaType
//

/// Media type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaType {
    /// Main segment.
    main: MediaTypeSegment,

    /// Subtype segment.
    subtype: MediaTypeSegment,
}

impl MediaType {
    /// Constructor.
    pub const fn new(main: MediaTypeSegment, subtype: MediaTypeSegment) -> Self {
        Self { main, subtype }
    }

    /// Constructor.
    pub fn new_owned(main: String, subtype: String) -> Self {
        Self::new(main.into(), subtype.into())
    }

    /// Constructor.
    pub const fn new_static(main: &'static str, subtype: &'static str) -> Self {
        Self::new(MediaTypeSegment::new_static(main), MediaTypeSegment::new_static(subtype))
    }
}

impl IntoOwned for MediaType {
    fn into_owned(self) -> Self {
        match self.main {
            MediaTypeSegment(Foster::Owned(_)) => match self.subtype {
                MediaTypeSegment(Foster::Owned(_)) => {
                    // Both main and subtype are already owned
                    self
                }

                MediaTypeSegment(Foster::Fostered(subtype)) => {
                    // Main is owned, subtype isn't
                    Self::new(self.main, subtype.into())
                }
            },

            MediaTypeSegment(Foster::Fostered(main)) => match self.subtype {
                MediaTypeSegment(Foster::Owned(_)) => {
                    // Subtype is owned, main isn't
                    Self::new(main.into(), self.subtype)
                }

                MediaTypeSegment(Foster::Fostered(subtype)) => {
                    // Both are not owned
                    Self::new_owned(main.into(), subtype.into())
                }
            },
        }
    }
}

impl IntoHeaderValue for MediaType {
    fn into_header_value(self) -> HeaderValue {
        HeaderValue::from_str(&self.to_string()).expect("should be safe")
    }
}

impl Parseable for MediaType {
    fn parse(representation: &str) -> Option<Self> {
        let (main, subtype) = representation.split_once("/")?;
        Some(Self::new(MediaTypeSegment::parse(main)?, MediaTypeSegment::parse(subtype)?))
    }
}

impl Sized for MediaType {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaType>();
        SELF_SIZE + self.main.size() + self.subtype.size()
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.main, self.subtype)
    }
}
