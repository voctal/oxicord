use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

pub(super) fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let repr = parse_macro_input!(attr as syn::Ident);
    let input = parse_macro_input!(item as ItemEnum);
    let name = &input.ident;
    let extra_attrs = &input.attrs;

    let variants: Vec<_> = input
        .variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let disc = match &v.discriminant {
                Some((
                    _,
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(i),
                        ..
                    }),
                )) => i.clone(),
                _ => panic!("every discord_enum variant needs an explicit `= N` discriminant"),
            };
            (ident, disc)
        })
        .collect();

    let idents: Vec<_> = variants.iter().map(|(i, _)| i).collect();
    let discs: Vec<&syn::LitInt> = variants.iter().map(|(_, d)| d).collect();

    quote! {
        #(#extra_attrs)*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum #name {
            #(#idents,)*
            Unknown(#repr),
        }

        impl From<#repr> for #name {
            fn from(v: #repr) -> Self {
                match v {
                    #(#discs => Self::#idents,)*
                    other => Self::Unknown(other),
                }
            }
        }

        impl From<#name> for #repr {
            fn from(v: #name) -> Self {
                match v {
                    #(#name::#idents => #discs,)*
                    #name::Unknown(other) => other,
                }
            }
        }

        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                ::serde::Serialize::serialize(&#repr::from(*self), s)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                Ok(Self::from(#repr::deserialize(d)?))
            }
        }
    }
    .into()
}
