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
    derive(
        input,
        quote!(DrawAt),
        quote!(draw_at),
        vec![
            quote!(res),
            quote!(frame),
            quote!(pos),
        ],
        vec![
            quote!(&mut DrawResources),
            quote!(&mut Frame),
            quote!(graphics::Pnt2),
        ],
    )
}


pub fn derive(
    input:    TokenStream,
    trait_:   proc_macro2::TokenStream,
    method:   proc_macro2::TokenStream,
    arg_name: impl IntoIterator<Item=proc_macro2::TokenStream>,
    arg_ty:   impl IntoIterator<Item=proc_macro2::TokenStream>,
)
    -> TokenStream
{
    let struct_ = parse_macro_input!(input as ItemStruct);

    let name     = struct_.ident;
    let arg_name = arg_name.into_iter();
    let arg_ty   = arg_ty.into_iter();

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
                self.#field.#method(res, frame, pos);
            )
        });

    let tokens = quote!(
        impl #trait_ for #name {
            fn #method(&mut self,
                #(
                    #arg_name: #arg_ty,
                )*
            ) {
                #(#fields)*
            }
        }
    );

    TokenStream::from(tokens)
}
