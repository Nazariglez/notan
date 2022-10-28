extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};
use syn::{ItemFn, ReturnType};

mod handlers;
mod shaders;
mod state;

#[cfg(all(feature = "glsl-to-spirv", feature = "shaderc"))]
compile_error!(
    "feature \"glsl-to-spirv\" and feature \"shaderc\" cannot be enabled at the same time"
);

#[proc_macro_attribute]
pub fn notan_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    handle_main_func(input)
}

fn handle_main_func(input: ItemFn) -> TokenStream {
    let ident = input.sig.ident.clone();
    let void_ret = input.sig.output == ReturnType::Default;
    let ret: proc_macro2::TokenStream = if void_ret { "()" } else { "Result<(), String>" }
        .parse()
        .unwrap();

    let expand = quote! {
        #input

        #[no_mangle]
        pub extern fn notan_main() -> #ret {
            #[allow(clippy::main_recursion)]
            #ident()
        }
    };

    expand.into()
}
#[proc_macro]
pub fn handler(input: TokenStream) -> TokenStream {
    let inputs: Vec<_> = input.into_iter().collect();
    let input_sting = inputs
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let code = handlers::process_tokens(input_sting);
    code.parse().unwrap()
}

#[proc_macro_derive(AppState)]
pub fn state_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    state::impl_state_derive(&ast)
}

#[proc_macro]
pub fn vertex_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let content = input.value();
    let spirv = shaders::spirv_from(&content, shaders::ShaderType::Vertex).unwrap();

    shaders::source_from_spirv(spirv).unwrap()
}

#[proc_macro]
pub fn include_vertex_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let relative_path = input.value();
    let spirv = shaders::spirv_from_file(&relative_path, shaders::ShaderType::Vertex).unwrap();

    shaders::source_from_spirv(spirv).unwrap()
}

#[proc_macro]
pub fn fragment_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let content = input.value();
    let spirv = shaders::spirv_from(&content, shaders::ShaderType::Fragment).unwrap();

    shaders::source_from_spirv(spirv).unwrap()
}

#[proc_macro]
pub fn include_fragment_shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let relative_path = input.value();
    let spirv = shaders::spirv_from_file(&relative_path, shaders::ShaderType::Fragment).unwrap();

    shaders::source_from_spirv(spirv).unwrap()
}

#[proc_macro_attribute]
pub fn uniform(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let derive: DeriveInput = syn::parse(input.clone()).unwrap();
    let ident = derive.ident;
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(glsl_layout::Uniform)]
        #input

        impl ::notan::graphics::Uniform for #ident {}
    };
    output.into()
}
