use proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, ItemFn};

/// Expands the `#[zap::function]` attribute.
///
/// This macro intentionally performs no transformation.
///
/// The enclosing `#[zap::module]` macro is responsible for discovering
/// exported functions, generating ABI wrappers, and constructing the
/// module descriptor.
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    TokenStream::from(quote! {
        #function
    })
}
