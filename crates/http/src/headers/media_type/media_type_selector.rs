use super::{
    super::{super::cache::*, parseable::*, preferences::*},
    segment::*,
};

use {
    kutil_std::{borrow::*, foster::*},
    std::{cmp::*, fmt, hash::*},
};

//
// MediaTypeSelector
//

/// [MediaType](super::media_type::MediaType) selector.
///
/// Either of the segments can be [Any](Selector::Any). However, if `main` is [Any](Selector::Any),
/// `subtype` must also be [Any](Selector::Any). Use [Self::is_valid] to check for this.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaTypeSelector {
    /// Main segment.
    main: Selector<MediaTypeSegment>,

    /// Subtype segment.
    subtype: Selector<MediaTypeSegment>,
}

impl MediaTypeSelector {
    /// Any.
    pub const ANY: Self = Self::new(Selector::Any, Selector::Any);

    /// Constructor.
    pub const fn new(main: Selector<MediaTypeSegment>, subtype: Selector<MediaTypeSegment>) -> Self {
        Self { main, subtype }
    }

    /// Constructor.
    pub fn new_owned(main: String, subtype: String) -> Self {
        Self::new(Selector::Specific(main.into()), Selector::Specific(subtype.into()))
    }

    /// Constructor.
    pub const fn new_static(main: &'static str, subtype: &'static str) -> Self {
        Self::new(
            Selector::Specific(MediaTypeSegment::new_static(main)),
            Selector::Specific(MediaTypeSegment::new_static(subtype)),
        )
    }

    /// Whether we are valid.
    ///
    /// If `main` is [Any](Selector::Any), `subtype` must also be [Any](Selector::Any).
    pub fn is_valid(&self) -> bool {
        self.main.is_specific() || !self.subtype.is_specific()
    }
}

impl IsSpecific for MediaTypeSelector {
    fn is_specific(&self) -> bool {
        self.main.is_specific() && self.subtype.is_specific()
    }
}

impl IntoOwned for MediaTypeSelector {
    fn into_owned(self) -> Self {
        match self.main {
            Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => match self.subtype {
                Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => {
                    // Both main and subtype are already owned
                    self
                }

                Selector::Specific(MediaTypeSegment(Foster::Fostered(subtype))) => {
                    // Main is owned, subtype isn't
                    Self::new(self.main, Selector::Specific(subtype.into()))
                }
            },

            Selector::Specific(MediaTypeSegment(Foster::Fostered(main))) => match self.subtype {
                Selector::Any | Selector::Specific(MediaTypeSegment(Foster::Owned(_))) => {
                    // Subtype is owned, main isn't
                    Self::new(Selector::Specific(main.into()), self.subtype)
                }

                Selector::Specific(MediaTypeSegment(Foster::Fostered(subtype))) => {
                    // Both are not owned
                    Self::new_owned(main.into(), subtype.into())
                }
            },
        }
    }
}

impl Parseable for MediaTypeSelector {
    fn parse(representation: &str) -> Option<Self> {
        let (main, subtype) = representation.split_once("/")?;
        Some(Self::new(Selector::parse(main)?, Selector::parse(subtype)?))
    }
}

impl Sized for MediaTypeSelector {
    fn size(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSelector>();
        SELF_SIZE + self.main.size() + self.subtype.size()
    }
}

impl fmt::Display for MediaTypeSelector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.main, self.subtype)
    }
}
