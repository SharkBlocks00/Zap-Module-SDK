use crate::model::ExportedFunction;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Expands the `#[zap_export]` attribute.
///
/// This macro intentionally performs no generation logic itself,
/// but it validates that the function is compatible with Zap exports.
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);

    match ExportedFunction::parse("standalone", item_fn.clone()) {
        Ok(parsed) => {
            let clean = parsed.item;
            TokenStream::from(quote! {
                #clean
            })
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
