//
// DebugFormat
//

/// Debug format. Allows for variations of the debug representation.
///
/// There is no requirement for a [Debuggable](super::debuggable::Debuggable) to support the
/// various formats, thus it can only be used reliably when the types are known in advance to
/// support it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DebugFormat {
    /// Verbose implies a consistent full representation with no optional information omitted.
    Verbose,

    /// Reduced implies an optimized representation in which shorter variations may be used when
    /// possible, and some optional information may be omitted.
    ///
    /// This is the default format.
    #[default]
    Reduced,

    /// Compact implies a single-line format.
    Compact,
}
