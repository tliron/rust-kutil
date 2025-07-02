use super::branch::*;

use deluxe::*;

//
// StructAttribute
//

/// Struct-level attribute for `#[derive(Debuggable)]`.
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(debuggable))]
pub struct StructAttribute {
    /// Branch
    #[deluxe(default)]
    pub branch: Branch,

    /// Optional tag.
    #[deluxe(default)]
    pub tag: Option<syn::Expr>,
}
