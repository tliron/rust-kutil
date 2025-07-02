use deluxe::*;

//
// EnumAttribute
//

/// Enum-level attribute for `#[derive(FromStr)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(from_str))]
pub struct FromStrEnumAttribute {
    /// Error.
    ///
    /// Must implement From<&str>.
    ///
    /// Will default to kutil_std::string::ParseError.
    #[deluxe(default)]
    pub error: Option<syn::Type>,

    /// Lowercase
    #[deluxe(default)]
    pub lowercase: bool,
}
