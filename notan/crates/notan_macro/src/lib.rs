extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};
use syn::{ItemFn, ItemStruct};

mod shaders;
mod state;

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

fn combo(arr: &[String]) -> Vec<Vec<String>> {
    let mut combi: Vec<Vec<String>> = vec![];
    let mut temp: Vec<String> = vec![];
    let nn = num::pow::pow(0, 0);
    let slent = num::pow::pow(2, arr.len());

    for i in 0..slent {
        temp = vec![];
        for j in 0..arr.len() {
            if (i & num::pow::pow(2, j)) != 0 {
                temp.push(arr[j].clone());
            }
        }

        if temp.len() > 0 {
            combi.push(temp);
        }
    }

    combi

    // let mut combinations: Vec<_> = combi.iter()
    //     .map(|n| n.join(""))
    //     .collect();
    //
    // combinations.sort();
    // combinations
}

fn enum_generics(g: &[Vec<String>], r: &str) -> String {
    g.iter()
        .enumerate()
        .map(|(i, n)| {
            let gen = n
                .iter()
                .map(|gen| format!("&mut {}", gen))
                .collect::<Vec<_>>()
                .join(", ");
            format!("_{}(Box<dyn Fn({}) -> {}>)", i, gen, r)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn params_generics(g: &[String], r: &str) -> String {
    g.iter()
        .enumerate()
        .map(|(i, n)| format!("param_{}: &mut {}", i, n))
        .collect::<Vec<_>>()
        .join(",")
}

fn enum_callback_generics(g: &[Vec<String>], list: &[String]) -> String {
    g.iter()
        .enumerate()
        .map(|(i, n)| {
            let gen = n
                .iter()
                .map(|gen| {
                    let index = list.iter().position(|g| g == gen).unwrap();
                    format!("param_{}", index)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("_{}(cb) => cb({})", i, gen)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn trait_impl_generics(g: &[Vec<String>], cb_id: &str, handler_id: &str) -> String { //TODO
    g.iter()
        .enumerate()
        .map(|(i, n)| {
            let gen = n
                .iter()
                .map(|gen| format!("&mut {}", gen))
                .collect::<Vec<_>>()
                .join(", ");

            // format!("_{}(Box<dyn Fn({}) -> {}>)", i, gen, r)
            format!(r#"
        #[allow(unused_parens)]
        impl<F, S> {}<S, ({})> for F
        where
            F: Fn({}) -> S + 'static,
            S: AppState
        {{
            fn callback(self) -> {}<S> {{
                {}::_{}(Box::new(self))
            }}
        }}
    "#, handler_id, gen, gen, cb_id, cb_id, i)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[proc_macro]
pub fn handler(input: TokenStream) -> TokenStream {
    let inputs: Vec<_> = input.into_iter().collect();
    let generics: Vec<String> = inputs[1..].iter().map(|t| t.to_string()).collect();
    let combinations = combo(&generics[..generics.len() - 1]);
    let return_value = generics.last().unwrap().to_string();
    let callback_ident = format!("{}Callback", inputs[0]);
    let handler_ident = format!("{}Handler", inputs[0]);
    let generated_enum = format!(
        "pub enum {}<S> {{ {} }}",
        callback_ident,
        enum_generics(&combinations, &return_value)
    );

    let generated_impl = format!(
        r#"impl<S> {}<S> {{
            pub(crate) fn exec(&self, {}) -> {} {{
                use {}::*;
                match self {{
                   {}
                }}
            }}
        }}"#,
        callback_ident,
        params_generics(&generics, &return_value),
        return_value,
        callback_ident,
        enum_callback_generics(&combinations, &generics)
    );

    let generated_trait = format!(r#"
    pub trait {}<S, Params> {{
        fn callback(self) -> {}<S>;
    }}
    "#,
        handler_ident,
        callback_ident
    );

    let generated_trait_impl = trait_impl_generics(&combinations, &callback_ident, &handler_ident);

    let code = [generated_enum, generated_impl, generated_trait, generated_trait_impl].join("\n");

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
