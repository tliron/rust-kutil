use super::{super::attributes::*, generator::*};

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate field handler.
    pub fn generate_handle_field(&self, field: &Field, last: bool) -> TokenStream {
        let field_name = &field.name;
        let quoted_field_name = field.name.to_string().to_token_stream();

        let tag = match &field.attribute.tag {
            Some(tag) => quote! {
                #tag(self, #quoted_field_name, writer, prefix, theme)?;
            },
            None => TokenStream::new(),
        };

        let write_value = field.attribute.value_as.generate_write_value(&field.attribute.value_style);

        let (write_prefix, sub_prefix) = match self.struct_attribute.branch {
            Branch::Thin => {
                (quote! { prefix.write_with_branch(writer, #last)?; }, quote! { &prefix.with_branch(#last) })
            }
            Branch::Thick => (
                quote! { prefix.write_with_thick_branch(writer, #last)?; },
                quote! { &prefix.with_thick_branch(#last) },
            ),
            Branch::Double => (
                quote! { prefix.write_with_double_branch(writer, #last)?; },
                quote! { &prefix.with_double_branch(#last) },
            ),
        };

        let mut write = match &field.attribute.iter {
            Iter::None => quote! {
                ::std::write!(writer, " ")?;
                let child_prefix = #sub_prefix;
                #write_value
                #tag
            },

            Iter::Item => quote! {
                let item_prefix = #sub_prefix;
                let child_prefix = &item_prefix.with("  ");
                let mut empty = true;

                for item in value {
                    empty = false;

                    item_prefix.write_with(writer, "- ")?;
                    let value = item;
                    #write_value

                    #tag
                }

                if empty {
                    ::std::write!(writer, " {}", theme.delimiter.style("[]"))?;
                }
            },

            Iter::KeyValue => {
                let write_key = field.attribute.key_as.generate_write_value(&field.attribute.key_style);
                quote! {
                    let item_prefix = #sub_prefix;
                    let child_prefix = &item_prefix.with("  ");
                    let mut empty = true;

                    for (k, v) in value {
                        empty = false;

                        item_prefix.write_with(writer, "? ")?;
                        let value = k;
                        #write_key

                        item_prefix.write_with(writer, ": ")?;
                        let value = v;
                        #write_value

                        #tag
                    }

                    if empty {
                        ::std::write!(writer, " {}", theme.delimiter.style("{}"))?;
                    }
                }
            }
        };

        write = match &field.attribute.option {
            true => quote! {
                match &self.#field_name {
                    Some(value) => {
                        #write
                    },
                    None => ::std::write!(writer, " {}", theme.bare.style("None"))?,
                }
            },

            false => quote! {
                let value = &self.#field_name;
                #write
            },
        };

        quote! {
            #write_prefix
            ::std::write!(writer, "{}:", theme.meta.style(#quoted_field_name))?;
            #write
        }
    }
}
