use proc_macro::TokenStream;
use quote::quote;

mod discord_bitflags;
mod discord_enum;
mod discord_str_enum;

/// Creates a Discord bitflags, which serializes into a number,
/// and auto derives basic traits.
///
/// Note: unknown bits are kept when it deserializes.
#[proc_macro]
pub fn discord_bitflags(input: TokenStream) -> TokenStream {
    discord_bitflags::expand(input)
}

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
pub fn discord_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    discord_enum::expand(attr, input)
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
    discord_str_enum::expand(item)
}
