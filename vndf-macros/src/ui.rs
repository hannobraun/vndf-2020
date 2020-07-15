use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Fields,
    FieldsNamed,
    FieldsUnnamed,
    Index,
    ItemStruct,
    punctuated::Pair,
};


pub fn derive_draw_at(input: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(input as ItemStruct);

    let name = struct_.ident;

    let fields = match struct_.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            Some(named)
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            Some(unnamed)
        }
        Fields::Unit => {
            None
        }
    };

    let fields = fields
        .into_iter()
        .map(|punctuated| {
            punctuated
                .into_pairs()
                .map(|pair| {
                    match pair {
                        Pair::Punctuated(field, _) => { field }
                        Pair::End(field)           => { field }
                    }
                })
        })
        .flatten()
        .enumerate()
        .map(|(i, field)| {
            let field = match field.ident {
                Some(ident) => {
                    quote!(#ident)
                }
                None => {
                    let i = Index::from(i);
                    quote!(#i)
                }
            };

            quote!(
                self.#field.draw_at(res, frame, pos);
            )
        });

    let tokens = quote!(
        impl DrawAt for #name {
            fn draw_at(&mut self,
                res:   &mut DrawResources,
                frame: &mut Frame,
                pos:   graphics::Pnt2,
            ) {
                #(#fields)*
            }
        }
    );

    TokenStream::from(tokens)
}
