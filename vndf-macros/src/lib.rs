use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{
    parse_macro_input,
    Ident,
    LitStr,
    Result,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
};


#[proc_macro]
pub fn keys(input: TokenStream) -> TokenStream {
    let Keys { keys } = parse_macro_input!(input as Keys);

    let serialize_match = (&keys).into_iter().map(|Key { name, kind, key }| {
        let keycode_or_mousebutton = keyboard_or_mousebutton(kind);

        quote!(
            Key::#kind(#keycode_or_mousebutton::#key) => #name,
        )
    });

    let visit_str_match = (&keys).into_iter().map(|Key { name, kind, key }| {
        let keycode_or_mousebutton = keyboard_or_mousebutton(kind);

        quote!(
            #name => Ok(Key::#kind(#keycode_or_mousebutton::#key)),
        )
    });

    let display_match = (&keys).into_iter().map(|Key { name, kind, key }| {
        let keycode_or_mousebutton = keyboard_or_mousebutton(kind);

        quote!(
            Key::#kind(#keycode_or_mousebutton::#key) => write!(f, "{}", #name),
        )
    });

    let tokens = quote!(
        impl Serialize for Key {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                #[allow(unused_parens)]
                let expr = match self {
                    #(#serialize_match)*

                    key => panic!("Variant not allowed: {:?}", key),
                };

                serializer.serialize_str(expr)
            }
        }

        impl<'de> Deserialize<'de> for Key {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                deserializer.deserialize_str(KeyVisitor)
            }
        }


        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a key identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error,
            {
                #[allow(unused_parens)]
                match value {
                    #(#visit_str_match)*

                    _ =>
                        Err(
                            E::custom(
                                format!("not a key identifier: {}", value)
                            )
                        )
                }
            }
        }

        impl fmt::Display for Key {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                #[allow(unused_parens)]
                match self {
                    #(#display_match)*

                    _ => write!(f, "Unknown key"),
                }
            }
        }
    );

    TokenStream::from(tokens)
}


struct Keys {
    pub keys: Punctuated<Key, Token![;]>,
}

impl Parse for Keys {
    fn parse(input: ParseStream) -> Result<Self> {
        let keys = input.parse_terminated::<_, Token![;]>(Key::parse)?;
        Ok(
            Self { keys }
        )
    }
}


struct Key {
    pub name: LitStr,
    pub kind: Ident,
    pub key:  Ident,
}

impl Parse for Key {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let kind = input.parse()?;
        input.parse::<Token![,]>()?;

        let key = input.parse()?;

        Ok(
            Self {
                name,
                kind,
                key,
            }
        )
    }
}


fn keyboard_or_mousebutton(kind: &Ident) -> proc_macro2::TokenStream {
    match kind.to_string().as_str() {
        "Keyboard" => quote!(KeyCode),
        "Mouse"    => quote!(MouseButton),
        kind       => panic!("Unexpected key kind `{}`", kind),
    }
}
