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

#[proc_macro_derive(ProcessInputAt)]
pub fn derive_process_input_at(input: TokenStream) -> TokenStream {
    ui::derive_process_input_at(input)
}

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    ui::derive_size(input)
}
