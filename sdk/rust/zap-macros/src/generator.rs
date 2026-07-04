use crate::model::ParsedModule;
use crate::util::{c_string, invoke_name};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(module: &ParsedModule) -> TokenStream {
    let wrappers = generate_wrappers(module);
    let metadata = generate_metadata(module);
    let arrays = generate_arrays(module);
    let init = generate_init(module);

    let module_ident = &module.ident;
    let passthrough = &module.passthrough_items;

    quote! {
        mod #module_ident {

            use ::zapsdk::IntoZapValue;

            #(#passthrough)*

            #wrappers

            #metadata

            #arrays

            #init
        }
    }
}

fn generate_wrappers(module: &ParsedModule) -> TokenStream {
    let wrappers = module.functions.iter().map(|func| {
        let wrapper_name = &func.wrapper_name;
        let original_name = &func.rust_name;
        let invoke = invoke_name(func.arity);

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #wrapper_name(
                args: *const ::zap_sdk::ZapValue,
                argc: u32,
            ) -> ::zap_sdk::ZapValue {
                ::zapsdk::wrapper::#invoke(args, argc, #original_name)
            }
        }
    });

    quote! {
        #(#wrappers)*
    }
}

fn generate_constant_inits(module: &ParsedModule) -> Vec<TokenStream> {
    module
        .constants
        .iter()
        .map(|constant| {
            let export = c_string(&constant.export_name);
            let value = &constant.rust_name;
            quote! {
                ::zap_sdk::ZapConstant {
                    name: #export.as_ptr() as *const i8,
                    value: #value.into_zap(),
                }
            }
        })
        .collect()
}

fn generate_metadata(module: &ParsedModule) -> TokenStream {
    let function_meta = module.functions.iter().map(|func| {
        let metadata = &func.metadata_name;
        let wrapper = &func.wrapper_name;
        let export = c_string(&func.export_name);
        let arity = func.arity as u32;
        quote! {
            const #metadata: ::zap_sdk::ZapFunction = ::zap_sdk::ZapFunction {
                name: #export.as_ptr() as *const i8,
                arity: #arity,
                function: #wrapper as *const ::std::ffi::c_void,
            };
        }
    });
    quote! {
        #(#function_meta)*
    }
}

fn generate_arrays(module: &ParsedModule) -> TokenStream {
    let func_count = module.functions.len();
    let functions = module.functions.iter().map(|f| &f.metadata_name);
    quote! {
        static FUNCTIONS: [::zap_sdk::ZapFunction; #func_count] = [
            #(#functions),*
        ];
    }
}

fn generate_init(module: &ParsedModule) -> TokenStream {
    let function_count = module.functions.len() as u32;
    let constant_count = module.constants.len() as u32;
    let constant_inits = generate_constant_inits(module);
    quote! {
        #[no_mangle]
        pub extern "C" fn module_init() -> *mut ::zap_sdk::ZapModule {
            let constants: &'static [::zap_sdk::ZapConstant] = Box::leak(Box::new([
                #(#constant_inits),*
            ]));

            let module = ::zap_sdk::ZapModule {
                abi_version: ::zap_sdk::ABI_VERSION,
                function_count: #function_count,
                functions: FUNCTIONS.as_ptr(),
                constant_count: #constant_count,
                constants: constants.as_ptr(),
            };

            Box::leak(Box::new(module)) as *mut _
        }
    }
}
