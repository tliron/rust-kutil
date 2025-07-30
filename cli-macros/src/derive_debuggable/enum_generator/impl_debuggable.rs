use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Debuggable`.
    pub fn generate_impl_debuggable(&self) -> TokenStream {
        let mut segments = Vec::default();

        for variant in &self.variants {
            segments.push(self.generate_handle_variant(variant));
        }

        let enum_name = &self.enum_name;
        let (impl_generics, struct_generics, where_clause) = self.enum_generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                ::kutil_cli::debug::Debuggable
                for #enum_name #struct_generics
                #where_clause
            {
                fn
                    write_debug_for
                    <WriteT>
                    (
                        &self,
                        writer: &mut WriteT,
                        context: &::kutil_cli::debug::DebugContext,
                    )
                    -> ::std::io::Result<()>
                    where WriteT: ::std::io::Write
                {
                    let context = &context.child().with_separator(true);

                    match self {
                        #(#segments)*
                    }

                    Ok(())
                }
            }
        }
    }
}
