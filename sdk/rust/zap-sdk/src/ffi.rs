use std::ffi::CString;
use std::sync::{Mutex, OnceLock};

//! Zap ABI definitions

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
    pub fn null() -> Self {
        Self {
            ty: TYPE_NULL,
            data: ZapValueData { integer: 0 },
            flags: 0,
            reserved: [0; 6],
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self {
            ty: TYPE_BOOL,
            data: ZapValueData { boolean: value },
            flags: 0,
            reserved: [0; 6],
        }
    }

    pub fn int(value: i64) -> Self {
        Self {
            ty: TYPE_INT,
            data: ZapValueData { integer: value },
            flags: 0,
            reserved: [0; 6],
        }
    }

    pub fn float(value: f64) -> Self {
        Self {
            ty: TYPE_FLOAT,
            data: ZapValueData { floating: value },
            flags: 0,
            reserved: [0; 6],
        }
    }

    pub fn string(ptr: *const i8, len: u64) -> Self {
        Self {
            ty: TYPE_STRING,
            data: ZapValueData {
                string: ZapString {
                    data: ptr,
                    length: len,
                },
            },
            flags: 0,
            reserved: [0; 6],
        }
    }

    pub fn owned_string<S: Into<String>>(value: S) -> Self {
        let string = CString::new(value.into())
            .expect("Zap strings cannot contain interior null bytes.");

        let len = string.as_bytes().len();
        let ptr = string.as_ptr();

        // Keep the CString alive for the lifetime of the module.
        string_storage().lock().unwrap().push(string);

        Self::string(ptr, len as u64)
    }
}
