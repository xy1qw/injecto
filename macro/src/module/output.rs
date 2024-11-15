use super::ModuleInput;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn expand(input: &ModuleInput) -> TokenStream {
    let ModuleInput {
        vis,
        name,
        functions,
    } = input;

    let functions_code = functions.iter().map(|(func, attrs)| {
        let func_name = &func.sig.ident;

        let static_func_name = format_ident!("{}_", func_name);
        let func_args = &func.sig.inputs;
        let func_output = &func.sig.output;
        let func_block = &func.block;

        // Проверка, что каждая функция имеет либо `#[singleton]`, либо `#[injectable]`
        if !(attrs.is_singleton || attrs.is_injectable) {
            return quote! {
                compile_error!("Functions within `module!` must have either #[singleton] or #[injectable] attribute.");
            };
        }

        quote! {
            fn #static_func_name(&self, #func_args) #func_output #func_block
        }
    });

    quote! {
        #[derive(Debug)]
        #vis struct #name;

        impl #name {
            pub fn new() -> Self {
                Self {}
            }

            #(#functions_code)*
        }
    }
}
