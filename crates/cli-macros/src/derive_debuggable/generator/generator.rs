use super::super::attributes::*;

use {deluxe::*, proc_macro2::*, quote::*, syn::spanned::*};

//
// Generator
//

/// Generator for `#[derive(Resolve)]`.
#[derive(Default)]
pub struct Generator {
    /// Name of the struct for which we are generating.
    pub struct_name: TokenStream,

    /// The generics of the struct for which we are generating.
    pub struct_generics: syn::Generics,

    /// Struct-level attribute.
    pub struct_attribute: StructAttribute,

    /// The debuggable fields.
    pub debuggable_fields: Vec<Field>,
}

impl Generator {
    /// Generate.
    pub fn generate(item: &mut syn::ItemStruct) -> syn::Result<TokenStream> {
        let generator = Generator::new(item)?;
        Ok(generator.generate_impl_debuggable())
    }

    /// Constructor.
    pub fn new(item: &mut syn::ItemStruct) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.struct_name = item.ident.to_token_stream();
        generator.struct_generics = item.generics.clone();

        generator.struct_attribute = extract_attributes(item)?;

        for field in item.fields.iter_mut() {
            let field_attribute: FieldAttribute = extract_attributes(field)?;

            if !matches!(field_attribute.iter, Iter::KeyValue)
                && !matches!(field_attribute.key_as, As::Display)
                && !matches!(field_attribute.key_style, Style::None)
            {
                return Err(syn::Error::new(
                    field.span(),
                    "`debuggable` attribute: cannot use key_as and key_style without iter(kv)",
                ));
            }

            if matches!(field_attribute.value_as, As::Debuggable) && !matches!(field_attribute.value_style, Style::None)
            {
                return Err(syn::Error::new(
                    field.span(),
                    "`debuggable` attribute: cannot use as(debuggable) with style",
                ));
            }

            if matches!(field_attribute.key_as, As::Debuggable) && !matches!(field_attribute.key_style, Style::None) {
                return Err(syn::Error::new(
                    field.span(),
                    "`debuggable` attribute: cannot use key_as(debuggable) with key_style",
                ));
            }

            if field_attribute.skip {
                continue;
            }

            let field_name = match &field.ident {
                Some(name) => name,
                None => return Err(syn::Error::new(field.span(), "`debuggable` attribute: unnamed field")),
            };

            generator.debuggable_fields.push(Field { name: field_name.to_token_stream(), attribute: field_attribute });
        }

        Ok(generator)
    }
}

//
// Field
//

/// Generator field.
pub struct Field {
    /// Field name.
    pub name: TokenStream,

    /// Field attribute.
    pub attribute: FieldAttribute,
}
