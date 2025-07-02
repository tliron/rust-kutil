use deluxe::*;

//
// EnumAttribute
//

/// Enum-level attribute for `#[derive(Display)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(display))]
pub struct DisplayEnumAttribute {
    /// Lowercase
    #[deluxe(default)]
    pub lowercase: bool,
}
