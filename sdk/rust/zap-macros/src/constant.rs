use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemConst};
use crate::model::ExportedConstant;

/// Expands the `#[zap_constant]` attribute.
///
/// This macro intentionally performs no generation logic itself,
/// but it validates that the constant is compatible with Zap exports.
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_const = parse_macro_input!(item as ItemConst);

    match ExportedConstant::parse("standalone", item_const.clone()) {
        Ok(parsed) => {
            let clean = parsed.item;
            TokenStream::from(quote! {
                #clean
            })
        }
        Err(err) => {
            TokenStream::from(err.to_compile_error())
        }
    }
}
