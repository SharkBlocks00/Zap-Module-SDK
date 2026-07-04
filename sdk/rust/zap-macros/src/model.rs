use proc_macro2::Span;
use syn::{FnArg, Ident, ItemConst, ItemFn, Pat, ReturnType, Type};

pub struct ExportedFunction {
    pub rust_name: Ident,
    pub wrapper_name: Ident,
    pub meta_name: Ident,
    pub export_name: String,

    pub parameters: Vec<FunctionParameter>,

    pub return_type: Option<Type>,
}

pub struct FunctionParameter {
    pub name: Ident,
    pub ty: Type,
}

pub struct ExportedConstant {
    pub rust_name: Ident,
    pub meta_name: Ident,
    pub export_name: String,

    pub ty: Type,
}

impl ExportedFunction {
    pub fn parse(function: &ItemFn) -> syn::Result<Self> {
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

                    parameters.push(FunctionParameter {
                        name: pattern.ident.clone(),
                        ty: (*arg.ty).clone(),
                    });
                }
            }
        }

        let wrapper_name = Ident::new(
            &format!("__zap_wrapper_{}", function.sig.ident),
            Span::call_site(),
        );

        let meta_name = Ident::new(
            &format!("__zap_meta_function_{}", function.sig.ident),
            Span::call_site(),
        );

        let return_type = match &function.sig.output {
            ReturnType::Default => None,

            ReturnType::Type(_, ty) => Some((**ty).clone()),
        };

        Ok(Self {
            rust_name: function.sig.ident.clone(),

            wrapper_name,

            meta_name,

            export_name: function.sig.ident.to_string(),

            parameters,

            return_type,
        })
    }

    pub fn arity(&self) -> usize {
        self.parameters.len()
    }
}

impl ExportedConstant {
    pub fn parse(item: &ItemConst) -> syn::Result<Self> {
        let meta_name = Ident::new(
            &format!("__zap_meta_constant_{}", item.ident),
            Span::call_site(),
        );

        Ok(Self {
            rust_name: item.ident.clone(),

            meta_name,

            export_name: item.ident.to_string(),

            ty: (*item.ty).clone(),
        })
    }
}
