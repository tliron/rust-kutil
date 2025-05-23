use {deluxe::*, proc_macro2::*, quote::*};

//
// FieldAttribute
//

/// Field-level attribute for `#[derive(Debuggable)]`.
#[derive(ExtractAttributes)]
#[deluxe(attributes(debuggable))]
pub struct FieldAttribute {
    /// Whether this field should be skipped.
    #[deluxe(default)]
    pub skip: bool,

    /// How to represent the value.
    #[deluxe(default, rename = as)]
    pub value_as: As,

    /// How to represent the key, for `iter(kv)`.
    #[deluxe(default)]
    pub key_as: As,

    /// Value style.
    #[deluxe(default, rename = style)]
    pub value_style: Style,

    /// Key style.
    #[deluxe(default)]
    pub key_style: Style,

    /// Whether it's an [Option].
    #[deluxe(default)]
    pub option: bool,

    /// Whether and how to iterate.
    #[deluxe(default)]
    pub iter: Iter,

    /// Optional tag.
    #[deluxe(default)]
    pub tag: Option<syn::Expr>,
}

//
// As
//

#[derive(Default, ParseMetaItem)]
pub enum As {
    #[default]
    Debug,
    DebugAlt,
    Display,
    Debuggable,
    Custom(syn::Expr),
}

impl As {
    /// Write it.
    pub fn generate_write_value(&self, style: &Style) -> TokenStream {
        match self {
            As::Debug => {
                let value = style.style(quote! { format!("{:?}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::DebugAlt => {
                let value = style.style(quote! { format!("{:#?}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::Display => {
                let value = style.style(quote! { format!("{}", value) });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::Debuggable => quote! {
                value.write_debug_for(writer, child_context)?;
            },

            As::Custom(custom) => {
                let value = style.style(quote! { (#custom)(value)? });
                quote! {
                    child_context.separate(writer)?;
                    ::std::write!(writer, "{}", #value)?;
                }
            }
        }
    }
}

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

//
// Iter
//

#[derive(Default, ParseMetaItem)]
pub enum Iter {
    #[default]
    None,
    Item,
    #[deluxe(rename = kv)]
    KeyValue,
}
