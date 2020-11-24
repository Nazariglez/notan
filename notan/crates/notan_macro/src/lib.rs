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

/*
let vertex = shader! {
    typ: "vertex",
    source: "vert.glsl"
};

let shader = shader! {
    vertex: "vert.glsl",
    fragment: "frag.glsl"
};

let shader = shader! {
    vertex: r#"
#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec4 a_color;

layout(location = 0) out vec4 v_color;
layout(location = 0) uniform mat4 u_matrix;

void main() {
    v_color = a_color;
    gl_Position = u_matrix * a_position;
}
    "#,

    fragment: r#"
#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

void main() {
    color = v_color;
}
    "#
}
*/
