//! Zap ABI definitions

use crate::ZapValue;
use crate::ZapValueData;
use std::ffi::CString;
use std::sync::{Mutex, OnceLock};

pub const ABI_VERSION: u32 = 1;

// types

pub const TYPE_NULL: u8 = 0;
pub const TYPE_INT: u8 = 1;
pub const TYPE_FLOAT: u8 = 2;
pub const TYPE_BOOL: u8 = 3;
pub const TYPE_STRING: u8 = 4;
pub const TYPE_ARRAY: u8 = 5;
pub const TYPE_OBJECT: u8 = 6;
pub const TYPE_FUNCTION: u8 = 7;

// status codes

pub const STATUS_OK: u32 = 0;
pub const STATUS_ERROR: u32 = 1;
pub const STATUS_PANIC: u32 = 2;

static STRING_STORAGE: OnceLock<Mutex<Vec<CString>>> = OnceLock::new();

fn string_storage() -> &'static Mutex<Vec<CString>> {
    STRING_STORAGE.get_or_init(|| Mutex::new(Vec::new()))
}
impl ZapValue {
    pub fn owned_string<S: Into<String>>(value: S) -> Self {
        let string =
            CString::new(value.into()).expect("Zap strings cannot contain interior null bytes.");

        let len = string.as_bytes().len();
        let ptr = string.as_ptr();

        // Keep the CString alive for the lifetime of the module.
        string_storage().lock().unwrap().push(string);

        Self::string(ptr, len as u64)
    }
}
