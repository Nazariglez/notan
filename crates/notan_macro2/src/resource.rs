extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_resource_id_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<u64> for #name {
            fn from(value: u64) -> Self {
                #name(value)
            }
        }

        impl Into<u64> for #name {
            fn into(self) -> u64 {
                self.0
            }
        }
    };
    gen.into()
}
