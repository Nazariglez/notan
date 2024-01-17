extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_state_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ::notan::core::AppState for #name #ty_generics #where_clause {}
        impl #impl_generics ::notan::core::storage::FromStorage<#name #ty_generics> for #name #ty_generics #where_clause {
            fn from_storage<'gk_state>(storage: &'gk_state mut ::notan::core::storage::Storage<#name #ty_generics>) -> &'gk_state mut Self {
                &mut storage.state
            }
        }
    };
    gen.into()
}
