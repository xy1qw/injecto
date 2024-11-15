mod module;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as module::ModuleInput);
    module::expand(&input).into()
}
