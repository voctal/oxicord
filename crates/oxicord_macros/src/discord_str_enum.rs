use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ItemEnum, Lit, LitStr, parse_macro_input};

/// See the docs in lib.rs
pub(super) fn expand(item: TokenStream) -> TokenStream {
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
