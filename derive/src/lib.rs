extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn nae_start(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    handle_func(input)
}

fn handle_func(input: ItemFn) -> TokenStream {
    let ident = input.ident.clone();
    let expand = quote! {
        #input

        #[no_mangle]
        pub extern fn nae_start() {
            #ident();
        }
    };

    expand.into()
}
