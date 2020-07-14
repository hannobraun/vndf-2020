mod keys;


use proc_macro::TokenStream;


#[proc_macro]
pub fn keys(input: TokenStream) -> TokenStream {
    keys::keys(input)
}
