mod derive_debuggable;

// See: https://petanode.com/posts/rust-proc-macro/

/// Procedural macro for `#[derive(Debuggable)]`.
#[proc_macro_derive(Debuggable, attributes(debuggable))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item: syn::ItemStruct = syn::parse_macro_input!(input);
    derive_debuggable::Generator::generate(&mut item).unwrap_or_else(|e| e.to_compile_error()).into()
}
