// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities for CLI programs.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

mod derive_debuggable;

use derive_debuggable::*;

// See: https://petanode.com/posts/rust-proc-macro/

/// Procedural macro for `#[derive(Debuggable)]`.
#[proc_macro_derive(Debuggable, attributes(debuggable))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse_macro_input!(input);

    match input.data {
        syn::Data::Struct(_) => StructGenerator::generate(&mut input),

        syn::Data::Enum(_) => EnumGenerator::generate(&mut input),

        _ => Err(syn::Error::new(input.ident.span(), "`Debuggable`: not a struct")),
    }
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
