use {deluxe::*, proc_macro2::*, quote::*};

//
// Style
//

#[derive(Default, ParseMetaItem)]
pub enum Style {
    #[default]
    None,
    Symbol,
    Number,
    String,
    Name,
    Meta,
    Error,
    Delimiter,
    Heading,
}

impl Style {
    /// Write it with style.
    pub fn style(&self, value: TokenStream) -> TokenStream {
        match self {
            Style::None => value,
            Style::Symbol => quote! { context.theme.symbol(#value) },
            Style::Number => quote! { context.theme.number(#value) },
            Style::String => quote! { context.theme.string(#value) },
            Style::Name => quote! { context.theme.name(#value) },
            Style::Meta => quote! { context.theme.meta(#value) },
            Style::Error => quote! { context.theme.error(#value) },
            Style::Delimiter => quote! { context.theme.delimiter(#value) },
            Style::Heading => quote! { context.theme.heading(#value) },
        }
    }
}
