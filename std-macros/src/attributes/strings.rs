use {deluxe::*, proc_macro2::*};

//
// StringsAttribute
//

/// Variant-level attribute for `#[derive(Display)]` and `#[derive(FromStr)]`.
///
/// If empty will default to the variant name.
///
/// The first expression will be used for `Display`.
///
/// ```
/// #[derive(Display)]
/// enum MyEnum {
///   #[strings("var", "variant")] // this
///   my_variant(Value)
/// }
/// ```
#[derive(Clone, ExtractAttributes)]
#[deluxe(attributes(strings))]
pub struct StringsAttribute(#[deluxe(flatten)] pub Vec<syn::LitStr>);

//
// Variant
//

/// Generator variant.
pub struct Variant {
    /// Variant name.
    pub name: TokenStream,

    /// Variant strings.
    pub strings: Vec<syn::LitStr>,
}
