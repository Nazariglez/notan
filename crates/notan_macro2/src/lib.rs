use proc_macro::*;

mod resource;
mod state;

#[proc_macro_derive(AppState)]
pub fn state_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    state::impl_state_derive(&ast)
}

#[proc_macro_derive(ResourceId)]
pub fn resource_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    resource::impl_resource_id_derive(&ast)
}
