use super::super::super::attributes::*;

use {deluxe::*, proc_macro2::*, quote::*};

//
// Generator
//

/// Generator for `#[derive(FromStr)]`.
#[derive(Default)]
pub struct Generator {
    /// Name of the enum for which we are generating.
    pub enum_name: TokenStream,

    /// The generics of the enum for which we are generating.
    pub enum_generics: syn::Generics,

    /// Enum-level attribute.
    pub enum_attribute: FromStrEnumAttribute,

    /// The variants that should be displayed.
    pub display_variants: Vec<Variant>,
}

impl Generator {
    /// Generate.
    pub fn generate(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
        let generator = Self::new(input)?;
        let stream = generator.generate_impl_from_str();
        Ok(stream)
    }

    /// Constructor.
    pub fn new(input: &mut syn::DeriveInput) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.enum_name = input.ident.to_token_stream();
        generator.enum_generics = input.generics.clone();
        generator.enum_attribute = extract_attributes(input)?;

        match &mut input.data {
            syn::Data::Enum(data) => {
                for variant in data.variants.iter_mut() {
                    if !variant.fields.is_empty() {
                        return Err(syn::Error::new(
                            variant.ident.span(),
                            "`FromStr`: variants with fields are not supported",
                        ));
                    }

                    let strings_attribute: StringsAttribute = extract_attributes(variant)?;
                    let variant_name = &variant.ident;
                    let variant_strings = if !strings_attribute.0.is_empty() {
                        strings_attribute.0.clone()
                    } else {
                        vec![syn::LitStr::new(variant_name.to_string().as_str(), variant_name.span())]
                    };
                    generator
                        .display_variants
                        .push(Variant { name: variant_name.to_token_stream(), strings: variant_strings });
                }
            }

            _ => return Err(syn::Error::new(input.ident.span(), "`FromStr`: not an enum")),
        }

        Ok(generator)
    }
}
