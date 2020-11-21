extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    handle_main_func(input)
}

fn handle_main_func(input: ItemFn) -> TokenStream {
    let ident = input.sig.ident.clone();
    let expand = quote! {
        #input

        #[no_mangle]
        pub extern fn notan_main() {
            #ident();
        }
    };

    expand.into()
}

//https://github.com/rust-tutorials
//https://floooh.github.io/2017/05/15/oryol-spirv.html
//http://nercury.github.io/rust/opengl/tutorial/2018/07/11/opengl-in-rust-from-scratch-10-procedural-macros.html
//https://stackoverflow.com/questions/58246366/rust-macro-that-counts-and-generates-repetitive-struct-fields

#[proc_macro_attribute]
pub fn shader(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    handle_shader_struct(input)
}

fn handle_shader_struct(mut input: ItemStruct) -> TokenStream {
    //https://docs.rs/syn/1.0.48/syn/struct.ItemStruct.html
    let expand = quote! {
        #input
    };

    expand.into()
}
