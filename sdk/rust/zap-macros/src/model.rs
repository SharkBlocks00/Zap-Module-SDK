use proc_macro2::{Span, TokenStream};
use syn::{FnArg, Ident, ItemConst, ItemFn, Meta, Pat, ReturnType, Type};

pub struct ParsedModule {
    pub ident: Ident,
    pub functions: Vec<ExportedFunction>,
    pub constants: Vec<ExportedConstant>,
    pub passthrough_items: Vec<TokenStream>,
}

pub struct ExportedFunction {
    pub rust_name: Ident,
    pub export_name: String,
    pub wrapper_name: Ident,
    pub metadata_name: Ident,
    pub item: ItemFn,
    pub arity: usize,
    pub return_type: Option<Type>,
    pub parameters: Vec<Parameter>,
}

pub struct Parameter {
    pub ident: Ident,
    pub ty: Type,
}

pub struct ExportedConstant {
    pub rust_name: Ident,
    pub export_name: String,
    pub metadata_name: Ident,
    pub item: ItemConst,
    pub ty: Type,
}

impl ExportedFunction {
    pub fn parse(module_name: &str, function: ItemFn) -> syn::Result<Self> {
        if function.sig.asyncness.is_some() {
            return Err(syn::Error::new_spanned(
                &function.sig.asyncness,
                "Zap modules cannot export async functions",
            ));
        }

        if !function.sig.generics.params.is_empty() {
            return Err(syn::Error::new_spanned(
                &function.sig.generics,
                "Zap modules cannot export generic functions",
            ));
        }

        if function.sig.variadic.is_some() {
            return Err(syn::Error::new_spanned(
                &function.sig.variadic,
                "Variadic functions are not supported",
            ));
        }

        if function.sig.abi.is_some() {
            return Err(syn::Error::new_spanned(
                &function.sig.abi,
                "extern functions are not supported",
            ));
        }

        if function.sig.constness.is_some() {
            return Err(syn::Error::new_spanned(
                &function.sig.constness,
                "const functions are not supported",
            ));
        }

        let mut parameters = Vec::new();

        for argument in &function.sig.inputs {
            match argument {
                FnArg::Receiver(receiver) => {
                    return Err(syn::Error::new_spanned(
                        receiver,
                        "methods cannot be exported",
                    ));
                }

                FnArg::Typed(arg) => {
                    let Pat::Ident(pattern) = arg.pat.as_ref() else {
                        return Err(syn::Error::new_spanned(
                            &arg.pat,
                            "unsupported parameter pattern",
                        ));
                    };

                    parameters.push(Parameter {
                        ident: pattern.ident.clone(),
                        ty: (*arg.ty).clone(),
                    });
                }
            }
        }

        let mut export_name = function.sig.ident.to_string();
        let mut clean_function = function.clone();
        let mut clean_attrs = Vec::new();

        for attr in function.attrs.into_iter() {
            if attr.path().is_ident("zap_export") {
                if let Meta::List(list) = &attr.meta {
                    let _ = list.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            let value = meta.value()?;
                            let name_lit: syn::LitStr = value.parse()?;
                            export_name = name_lit.value();
                        }
                        Ok(())
                    });
                }
            } else {
                clean_attrs.push(attr);
            }
        }

        clean_function.attrs = clean_attrs;

        let wrapper_name = Ident::new(
            &format!("__zap_{}_wrapper_{}", module_name, function.sig.ident),
            Span::call_site(),
        );

        let metadata_name = Ident::new(
            &format!("__zap_{}_meta_function_{}", module_name, function.sig.ident),
            Span::call_site(),
        );

        let return_type = match &function.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => Some((**ty).clone()),
        };

        let arity = parameters.len();

        Ok(Self {
            rust_name: function.sig.ident.clone(),
            export_name,
            wrapper_name,
            metadata_name,
            item: clean_function,
            arity,
            return_type,
            parameters,
        })
    }
}

impl ExportedConstant {
    pub fn parse(module_name: &str, item: ItemConst) -> syn::Result<Self> {
        let mut export_name = item.ident.to_string();
        let mut clean_item = item.clone();
        let mut clean_attrs = Vec::new();

        for attr in item.attrs.into_iter() {
            if attr.path().is_ident("zap_constant") {
                if let Meta::List(list) = &attr.meta {
                    let _ = list.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            let value = meta.value()?;
                            let name_lit: syn::LitStr = value.parse()?;
                            export_name = name_lit.value();
                        }
                        Ok(())
                    });
                }
            } else {
                clean_attrs.push(attr);
            }
        }

        clean_item.attrs = clean_attrs;

        let metadata_name = Ident::new(
            &format!("__zap_{}_meta_constant_{}", module_name, item.ident),
            Span::call_site(),
        );

        Ok(Self {
            rust_name: item.ident.clone(),
            export_name,
            metadata_name,
            item: clean_item,
            ty: (*item.ty).clone(),
        })
    }
}
