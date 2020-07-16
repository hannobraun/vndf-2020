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


pub fn derive_draw(input: TokenStream) -> TokenStream {
    dispatch_to_all(
        input,
        quote!(Draw),
        quote!(draw),
        vec![
            quote!(res),
            quote!(frame),
        ],
        vec![
            quote!(&mut DrawResources),
            quote!(&mut Frame),
        ],
    )
}

pub fn derive_draw_at(input: TokenStream) -> TokenStream {
    dispatch_to_all(
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


pub fn dispatch_to_all(
    input:    TokenStream,
    trait_:   proc_macro2::TokenStream,
    method:   proc_macro2::TokenStream,
    arg_name: Vec<proc_macro2::TokenStream>,
    arg_ty:   Vec<proc_macro2::TokenStream>,
)
    -> TokenStream
{
    let struct_ = parse_macro_input!(input as ItemStruct);

    let name = &struct_.ident;

    let method_calls = dispatch_calls(&struct_, &method, &arg_name);

    let tokens = quote!(
        impl #trait_ for #name {
            fn #method(&mut self,
                #(
                    #arg_name: #arg_ty,
                )*
            ) {
                #(#method_calls)*
            }
        }
    );

    TokenStream::from(tokens)
}

fn dispatch_calls<'a>(
    struct_:  &'a ItemStruct,
    method:   &'a proc_macro2::TokenStream,
    arg_name: &'a [proc_macro2::TokenStream],
)
    -> impl Iterator<Item=proc_macro2::TokenStream> + 'a
{
    let fields = match &struct_.fields {
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

    fields
        .into_iter()
        .map(|punctuated| {
            punctuated
                .pairs()
                .map(|pair| {
                    match pair {
                        Pair::Punctuated(field, _) => { field }
                        Pair::End(field)           => { field }
                    }
                })
        })
        .flatten()
        .enumerate()
        .map(move |(i, field)| {
            let field = match &field.ident {
                Some(ident) => {
                    quote!(#ident)
                }
                None => {
                    let i = Index::from(i);
                    quote!(#i)
                }
            };

            quote!(
                self.#field.#method(#(#arg_name,)*);
            )
        })
}
