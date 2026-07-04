use proc_macro2::TokenStream;
use quote::quote;
use crate::model::ParsedModule;
use crate::util::{c_string, invoke_name};

pub fn generate(module: &ParsedModule) -> TokenStream {
    let wrappers = generate_wrappers(module);
    let metadata = generate_metadata(module);
    let arrays = generate_arrays(module);
    let init = generate_init(module);

    let module_ident = &module.ident;
    let passthrough = &module.passthrough_items;

    quote! {
        mod #module_ident {
            #(#passthrough)*

            // Module-specific internal SDK usage could be imported here if needed
            // But usually we rely on absolute paths.

            #wrappers

            #metadata

            #arrays

            #init
        }
    }
}

fn generate_wrappers(module: &ParsedModule) -> TokenStream {
    let mut streams = Vec::new();

    for func in &module.functions {
        let wrapper_name = &func.wrapper_name;
        let original_name = &func.rust_name;
        let invoke = invoke_name(func.arity);

        let wrapper = quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #wrapper_name(
                args: *const ::zap_sdk::ZapValue,
                argc: u32
            ) -> ::zap_sdk::ZapValue {
                ::zap::wrapper::#invoke(args, argc, #original_name)
            }
        };
        streams.push(wrapper);
    }

    quote! {
        #(#streams)*
    }
}

fn generate_metadata(module: &ParsedModule) -> TokenStream {
    let mut streams = Vec::new();

    for func in module.functions.iter() {
        let meta_name = &func.metadata_name;
        let export_name_c = c_string(&func.export_name);
        let arity = func.arity as u32;
        let wrapper_name = &func.wrapper_name;

        // Note: we might want ZapFunction to be constant
        let meta = quote! {
            const #meta_name: ::zap_sdk::module::ZapFunction = ::zap_sdk::module::ZapFunction {
                name: #export_name_c.as_ptr() as *const i8,
                arity: #arity,
                function: #wrapper_name as *const ::std::ffi::c_void,
            };
        };
        streams.push(meta);
    }

    for constant in module.constants.iter() {
        let meta_name = &constant.metadata_name;
        let export_name_c = c_string(&constant.export_name);
        let original_name = &constant.rust_name;

        // If ZapValue cannot be const-initialized via trait, we use lazy static evaluation. 
        // We will try lazy initialization later if a direct assignment fails.
        // Since we don't know the exact mechanism zap uses for constants, let's just assign.
        // If into_zap() isn't const, then `const #meta_name` won't compile, so we will generate it inline in the arrays or as a function if needed.
        // Actually, we'll follow user instructions directly first.
        let meta = quote! {
            const #meta_name: ::zap_sdk::module::ZapConstant = ::zap_sdk::module::ZapConstant {
                name: #export_name_c.as_ptr() as *const i8,
                value: ::zap::traits::IntoZapValue::into_zap(#original_name),
            };
        };
        streams.push(meta);
    }

    quote! {
        #(#streams)*
    }
}

fn generate_arrays(module: &ParsedModule) -> TokenStream {
    let func_count = module.functions.len();
    let const_count = module.constants.len();

    let func_names = module.functions.iter().map(|f| &f.metadata_name);
    let const_names = module.constants.iter().map(|c| &c.metadata_name);

    if const_count > 0 {
        // If there are constants, evaluate them lazily using a static method or lazy lock.
        // For standard Rust arrays:
        quote! {
            static FUNCTIONS: [::zap_sdk::module::ZapFunction; #func_count] = [
                #(#func_names),*
            ];

            // Warning: `into_zap` may not be const. Using a function returning the array. 
            // Wait! The user asked for exactly `static CONSTANTS: [ZapConstant; 3] = [ ... ];`
            // Let's output it exactly as requested.
            static CONSTANTS: [::zap_sdk::module::ZapConstant; #const_count] = [
                #(#const_names),*
            ];
        }
    } else {
        quote! {
            static FUNCTIONS: [::zap_sdk::module::ZapFunction; #func_count] = [
                #(#func_names),*
            ];

            static CONSTANTS: [::zap_sdk::module::ZapConstant; 0] = [];
        }
    }
}

fn generate_init(module: &ParsedModule) -> TokenStream {
    let func_count = module.functions.len() as u32;
    let const_count = module.constants.len() as u32;

    quote! {
        #[no_mangle]
        pub extern "C" fn zap_module_init() -> ::zap_sdk::module::ZapModule {
            ::zap_sdk::module::ZapModule {
                abi_version: ::zap_sdk::ffi::ABI_VERSION,
                function_count: #func_count,
                functions: FUNCTIONS.as_ptr(),
                constant_count: #const_count,
                constants: CONSTANTS.as_ptr(),
            }
        }
    }
}
