use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Item, ItemMod};
use crate::model::{ExportedConstant, ExportedFunction, ParsedModule};

pub fn parse(module: ItemMod) -> syn::Result<ParsedModule> {
    let ident = module.ident.clone();
    let module_name = ident.to_string();

    let mut functions = Vec::new();
    let mut constants = Vec::new();
    let mut passthrough_items = Vec::new();

    if let Some((_, items)) = module.content {
        for item in items {
            match item {
                Item::Fn(item_fn) => {
                    let has_export = item_fn.attrs.iter().any(|attr| attr.path().is_ident("zap_export"));
                    if has_export {
                        let parsed = ExportedFunction::parse(&module_name, item_fn)?;
                        passthrough_items.push(parsed.item.to_token_stream());
                        functions.push(parsed);
                    } else {
                        passthrough_items.push(item_fn.to_token_stream());
                    }
                }
                Item::Const(item_const) => {
                    let has_constant = item_const.attrs.iter().any(|attr| attr.path().is_ident("zap_constant"));
                    if has_constant {
                        let parsed = ExportedConstant::parse(&module_name, item_const)?;
                        passthrough_items.push(parsed.item.to_token_stream());
                        constants.push(parsed);
                    } else {
                        passthrough_items.push(item_const.to_token_stream());
                    }
                }
                other => {
                    passthrough_items.push(other.to_token_stream());
                }
            }
        }
    }

    Ok(ParsedModule {
        ident,
        functions,
        constants,
        passthrough_items,
    })
}
