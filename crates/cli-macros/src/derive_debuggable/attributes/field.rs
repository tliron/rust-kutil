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
    Display,
    #[default]
    Debug,
    DebugAlt,
    Debuggable,
}

impl As {
    /// Write it.
    pub fn generate_write_value(&self, style: &Style) -> TokenStream {
        match self {
            As::Display => {
                let value = style.style(quote! { format!("{}", value) });
                quote! {
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::Debug => {
                let value = style.style(quote! { format!("{:?}", value) });
                quote! {
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::DebugAlt => {
                let value = style.style(quote! { format!("{:#?}", value) });
                quote! {
                    ::std::write!(writer, "{}", #value)?;
                }
            }

            As::Debuggable => quote! {
                value.write_debug_representation(writer, child_prefix, theme)?;
            },
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
    Bare,
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
            Style::Bare => quote! { theme.bare.style(#value) },
            Style::Number => quote! { theme.number.style(#value) },
            Style::String => quote! { theme.string.style(#value) },
            Style::Name => quote! { theme.name.style(#value) },
            Style::Meta => quote! { theme.meta.style(#value) },
            Style::Error => quote! { theme.error.style(#value) },
            Style::Delimiter => quote! { theme.delimiter.style(#value) },
            Style::Heading => quote! { theme.heading.style(#value) },
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
