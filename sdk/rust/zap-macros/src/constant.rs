use proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, ItemConst};

/// Expands the `#[zap::constant]` attribute.
///
/// This macro intentionally performs no transformation.
///
/// The surrounding `#[zap::module]` macro will later inspect the module,
/// discover every item tagged with `#[zap::constant]`, generate the
/// necessary ABI conversion code, and export the constant automatically.
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let constant = parse_macro_input!(item as ItemConst);

    TokenStream::from(quote! {
        #constant
    })
}
