extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use syn::{ItemFn, ItemStruct};

mod shaders;

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

#[proc_macro]
pub fn vertex_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let content = input.value();
    let spirv = shaders::spirv_from(&content, shaders::ShaderType::Vertex).unwrap();
    let source = shaders::source_from_spirv(spirv).unwrap();

    source
}

#[proc_macro]
pub fn include_vertex_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let relative_path = input.value();
    let spirv = shaders::spirv_from_file(&relative_path, shaders::ShaderType::Vertex).unwrap();
    let source = shaders::source_from_spirv(spirv).unwrap();

    source
}

#[proc_macro]
pub fn fragment_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let content = input.value();
    let spirv = shaders::spirv_from(&content, shaders::ShaderType::Fragment).unwrap();
    let source = shaders::source_from_spirv(spirv).unwrap();

    source
}

#[proc_macro]
pub fn include_fragment_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let relative_path = input.value();
    let spirv = shaders::spirv_from_file(&relative_path, shaders::ShaderType::Fragment).unwrap();
    let source = shaders::source_from_spirv(spirv).unwrap();

    source
}
