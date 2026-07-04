mod constant;
mod export;
mod module;
mod util;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn zap_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    export::expand(attr, item)
}

#[proc_macro_attribute]
pub fn zap_constant(attr: TokenStream, item: TokenStream) -> TokenStream {
    constant::expand(attr, item)
}

#[proc_macro_attribute]
pub fn zap_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    module::expand(attr, item)
}
