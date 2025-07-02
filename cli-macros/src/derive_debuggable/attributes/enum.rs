use deluxe::*;

//
// EnumAttribute
//

/// Enum-level attribute for `#[derive(Debuggable)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(debuggable))]
pub struct EnumAttribute {
    /// Whether to include the variant name.
    #[deluxe(default = true)]
    pub variant: bool,
}
