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
    let mut bytes = name.as_bytes().to_vec();
    bytes.push(0); // Null-terminate for C string
    proc_macro2::Literal::byte_string(&bytes)
}
