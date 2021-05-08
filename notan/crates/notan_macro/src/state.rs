extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub(crate) fn impl_state_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl notan::app::AppState for #name {}
    };
    gen.into()
}
