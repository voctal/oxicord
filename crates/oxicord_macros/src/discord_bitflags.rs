use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, Token, Visibility, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct BitflagsInput {
    docs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    bits_ty: Ident,
    body: proc_macro2::TokenStream,
}

impl Parse for BitflagsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let docs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let bits_ty: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let body: proc_macro2::TokenStream = content.parse()?;

        Ok(BitflagsInput {
            docs,
            vis,
            ident,
            bits_ty,
            body,
        })
    }
}

/// See the docs in lib.rs
pub(super) fn expand(input: TokenStream) -> TokenStream {
    let BitflagsInput {
        docs,
        vis,
        ident,
        bits_ty,
        body,
    } = parse_macro_input!(input as BitflagsInput);

    let expanded = quote! {
        ::bitflags::bitflags! {
            #(#docs)*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #vis struct #ident: #bits_ty {
                #body
            }
        }

        impl ::serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                self.bits().serialize(serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let bits = #bits_ty::deserialize(deserializer)?;
                Ok(#ident::from_bits_retain(bits))
            }
        }
    };

    expanded.into()
}
