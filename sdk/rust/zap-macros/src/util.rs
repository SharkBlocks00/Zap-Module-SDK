use proc_macro2::TokenStream;
use quote::quote;

pub fn invoke_name(argument_count: usize) -> TokenStream {
    let name = syn::Ident::new(
        &format!("invoke{}", argument_count),
        proc_macro2::Span::call_site(),
    );

    quote! { #name }
}

pub fn c_string(name: &str) -> proc_macro2::Literal {
    proc_macro2::Literal::byte_string(name.as_bytes())
}
