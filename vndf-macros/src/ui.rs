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
        quote!(crate::frontend::ui::traits::Draw),
        quote!(draw),
        vec![
            quote!(res),
            quote!(frame),
        ],
        vec![
            quote!(&mut crate::frontend::drawers::DrawResources),
            quote!(&mut crate::frontend::drawers::Frame),
        ],
        quote!(Result<(), crate::frontend::ui::traits::DrawError>),
        quote!(Ok(())),
        CallKind::Result,
    )
}

pub fn derive_draw_at(input: TokenStream) -> TokenStream {
    dispatch_to_all(
        input,
        quote!(crate::frontend::ui::traits::DrawAt),
        quote!(draw_at),
        vec![
            quote!(res),
            quote!(frame),
            quote!(pos),
        ],
        vec![
            quote!(&mut crate::frontend::drawers::DrawResources),
            quote!(&mut crate::frontend::drawers::Frame),
            quote!(crate::graphics::Pnt2),
        ],
        quote!(Result<(), crate::frontend::ui::traits::DrawError>),
        quote!(Ok(())),
        CallKind::Result,
    )
}

pub fn derive_size(input: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(input as ItemStruct);

    let name     = &struct_.ident;
    let method   = quote!(size);

    let mut dispatch_calls: Vec<_> =
        dispatch_calls(&struct_, &method, &[], CallKind::Expression)
            .collect();

    if dispatch_calls.len() != 1 {
        panic!("Can only derive `Size`, if struct has exactly one field");
    }

    let dispatch_call = dispatch_calls.remove(0);

    let tokens = quote!(
        impl crate::frontend::ui::traits::Size for #name {
            fn size(&self) -> graphics::Size {
                #dispatch_call
            }
        }
    );

    TokenStream::from(tokens)
}


pub fn dispatch_to_all(
    input:    TokenStream,
    trait_:   proc_macro2::TokenStream,
    method:   proc_macro2::TokenStream,
    arg_name: Vec<proc_macro2::TokenStream>,
    arg_ty:   Vec<proc_macro2::TokenStream>,
    return_:  proc_macro2::TokenStream,
    result:   proc_macro2::TokenStream,
    kind:     CallKind,
)
    -> TokenStream
{
    let struct_ = parse_macro_input!(input as ItemStruct);

    let name = &struct_.ident;

    let method_calls = dispatch_calls(
        &struct_,
        &method,
        &arg_name,
        kind,
    );

    let tokens = quote!(
        impl #trait_ for #name {
            fn #method(&mut self,
                #(
                    #arg_name: #arg_ty,
                )*
            )
                -> #return_
            {
                #(#method_calls)*
                #result
            }
        }
    );

    TokenStream::from(tokens)
}

fn dispatch_calls<'a>(
    struct_:  &'a ItemStruct,
    method:   &'a proc_macro2::TokenStream,
    arg_name: &'a [proc_macro2::TokenStream],
    kind:     CallKind,
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

            match kind {
                CallKind::Expression => {
                    quote!(
                        self.#field.#method(#(#arg_name,)*)
                    )
                }
                CallKind::Result => {
                    quote!(
                        self.#field.#method(#(#arg_name,)*)?;
                    )
                }
            }
        })
}


pub enum CallKind {
    Expression,
    Result,
}
