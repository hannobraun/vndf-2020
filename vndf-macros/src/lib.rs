mod keys;
mod ui;


use proc_macro::TokenStream;


#[proc_macro]
pub fn keys(input: TokenStream) -> TokenStream {
    keys::keys(input)
}


#[proc_macro_derive(Draw)]
pub fn derive_draw(input: TokenStream) -> TokenStream {
    ui::derive_draw(input)
}

#[proc_macro_derive(DrawAt)]
pub fn derive_draw_at(input: TokenStream) -> TokenStream {
    ui::derive_draw_at(input)
}
