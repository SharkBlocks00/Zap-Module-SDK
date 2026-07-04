use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};
use crate::{generator, parser};

pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as ItemMod);

    match parser::parse(module) {
        Ok(parsed) => {
            let output = generator::generate(&parsed);
            TokenStream::from(output)
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
