use super::style::*;

use {deluxe::*, proc_macro2::*, quote::*};

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
    #[deluxe(rename = dyn_debuggable)]
    DynDebuggable,
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

            As::DynDebuggable => quote! {
                value.write_debug_for(::std::boxed::Box::new(writer), child_context)?;
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
