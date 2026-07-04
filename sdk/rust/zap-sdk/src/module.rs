use std::ffi::c_void;

use crate::ZapValue;

// Native function signature
pub type NativeFunction = extern "C" fn(*const ZapValue, u32) -> ZapValue;

// Exported function
#[repr(C)]
pub struct ZapFunction {
    name: *const i8,
    pub arity: u32,
    pub function: *const c_void,
}

// exported constants
#[repr(C)]
pub struct ZapConstant {
    pub name: *const i8,
    pub value: ZapValue,
}

// modules
#[repr(C)]
pub struct ZapModule {
    pub abi_version: u32,

    pub function_count: u32,
    pub functions: *const ZapFunction,

    pub constant_count: u32,
    pub constants: *const ZapConstant,
}

unsafe impl Sync for ZapFunction {}
unsafe impl Sync for ZapConstant {}
