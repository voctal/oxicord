use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ItemEnum, Lit, LitStr, parse_macro_input};

/// Automatically derives common traits to Discord API types structures.
#[proc_macro_attribute]
pub fn discord_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    quote! {
        #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        #input
    }
    .into()
}

/// Create a Discord non-exhaustive enum with an Unknown variant.
#[proc_macro_attribute]
pub fn discord_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
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

/// Like [`discord_enum`] but for string enums.
///
/// Variant names are converted to snake_case, but you can override
/// them use #[value = "..."] like this:
///
/// ```
/// # use oxicord_macros::discord_str_enum;
///
/// #[discord_str_enum]
/// pub enum Example {
///     TestA,
///     #[value = "another_name"]
///     TestB,
/// }
/// ```
#[proc_macro_attribute]
pub fn discord_str_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let name = &input.ident;
    let extra_attrs = &input.attrs;
    let vis = &input.vis;

    let mut idents = Vec::new();
    let mut values: Vec<LitStr> = Vec::new();
    let mut variant_attrs = Vec::new();

    for v in &input.variants {
        let mut explicit: Option<LitStr> = None;
        let mut kept_attrs = Vec::new();
        for attr in &v.attrs {
            if attr.path().is_ident("value") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = &nv.value
                    {
                        explicit = Some(s.clone());
                        continue;
                    }
                }
            }
            kept_attrs.push(attr.clone());
        }

        let value = explicit
            .unwrap_or_else(|| LitStr::new(&to_snake_case(&v.ident.to_string()), v.ident.span()));

        idents.push(v.ident.clone());
        values.push(value);
        variant_attrs.push(kept_attrs);
    }
    let idents = &idents;

    quote! {
        #(#extra_attrs)*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        #vis enum #name {
            #(
                #(#variant_attrs)*
                #idents,
            )*
            Unknown(::std::string::String),
        }

        impl #name {
            pub fn as_str(&self) -> &str {
                match self {
                    #(Self::#idents => #values,)*
                    Self::Unknown(s) => s.as_str(),
                }
            }
        }

        impl ::std::convert::From<&str> for #name {
            fn from(value: &str) -> Self {
                match value {
                    #(#values => Self::#idents,)*
                    other => Self::Unknown(other.to_string()),
                }
            }
        }

        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                let s = <::std::string::String as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                ::std::result::Result::Ok(Self::from(s.as_str()))
            }
        }
    }
    .into()
}

/// TODO: move to oxicord_utils or not?
///
/// TODO: tests
fn to_snake_case(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 4);
    let chars: Vec<char> = ident.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            if i > 0 && (prev_lower || next_lower) {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
