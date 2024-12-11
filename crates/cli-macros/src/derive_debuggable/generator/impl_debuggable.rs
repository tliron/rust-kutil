use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl Debuggable`.
    pub fn generate_impl_debuggable(&self) -> TokenStream {
        let mut segments = Vec::new();

        let mut iterator = self.debuggable_fields.iter().peekable();
        while let Some(debuggable_field) = iterator.next() {
            segments.push(self.generate_handle_field(debuggable_field, iterator.peek().is_none()));
        }

        let tag = match &self.struct_attribute.tag {
            Some(tag) => quote! {
                #tag(self, "", writer, prefix, theme)?;
            },
            None => TokenStream::new(),
        };

        let struct_name = &self.struct_name;
        let quoted_struct_name = struct_name.to_string().to_token_stream();
        let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                ::kutil_cli::debug::Debuggable
                for #struct_name #struct_generics
                #where_clause
            {
                fn
                    write_debug_representation
                    <WriteT>
                    (
                        &self,
                        writer: &mut WriteT,
                        prefix: &::kutil_cli::debug::DebugPrefix,
                        theme: &::kutil_cli::debug::Theme,
                    )
                    -> ::std::io::Result<()>
                    where WriteT: ::std::io::Write
                {
                    use ::owo_colors::OwoColorize;

                    ::std::write!(writer, "{}", theme.heading.style(#quoted_struct_name))?;

                    #tag

                    #(#segments)*

                    Ok(())
                }
            }
        }
    }
}
