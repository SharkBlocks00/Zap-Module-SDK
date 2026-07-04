use std::ffi::c_void;

use std::ffi::CStr;
use std::fmt;

use crate::*;

// utf-8 string
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZapString {
    pub data: *const i8,
    pub length: u64,
}

// generic value
#[repr(C)]
#[derive(Clone, Copy)]
pub union ZapValueData {
    pub integer: i64,
    pub floating: f64,
    pub boolean: bool,
    pub string: ZapString,
    pub pointer: *mut c_void,
}

// runtime values
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZapValue {
    pub ty: u8,
    pub data: ZapValueData,
    pub flags: u8,
    pub reserved: [u8; 6],
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

    pub fn boolean(value: bool) -> Self {
        Self {
            ty: TYPE_BOOL,
            data: ZapValueData { boolean: value },
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

    pub fn pointer(ptr: *mut c_void, ty: u8) -> Self {
        Self {
            ty,
            data: ZapValueData { pointer: ptr },
            flags: 0,
            reserved: [0; 6],
        }
    }
}

impl fmt::Display for ZapValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self.ty {
                TYPE_NULL => write!(f, "ZapValue(Null)"),
                TYPE_INT => write!(f, "ZapValue(Int: {})", self.data.integer),
                TYPE_FLOAT => write!(f, "ZapValue(Float: {})", self.data.floating),
                TYPE_BOOL => write!(f, "ZapValue(Bool: {})", self.data.boolean),
                TYPE_STRING => {
                    if self.data.string.data.is_null() {
                        write!(f, "ZapValue(String(NULL))")
                    } else {
                        let s = CStr::from_ptr(self.data.string.data).to_string_lossy();
                        write!(
                            f,
                            "ZapValue(String({:?}, {} bytes))",
                            s, self.data.string.length
                        )
                    }
                }
                TYPE_POINTER => write!(f, "ZapValue(Pointer: {:?})", self.data.pointer),

                TYPE_ARRAY => write!(f, "ZapValue(Array({:p}))", self.data.pointer),
                TYPE_OBJECT => write!(f, "ZapValue(Object({:p}))", self.data.pointer),
                TYPE_FUNCTION => write!(f, "ZapValue(Function({:p}))", self.data.pointer),
                other => write!(f, "ZapValue(UnknownType({}))", other),
            }
        }
    }
}

// SAFETY: sync is required so constant and function can live in static tables.
// IF ZapValue is EVER shared across threads while wrapping a live
// Array/Object/Function pointer, the CALLER is responsible for ensuring
// the underlying data is thread-safe.

unsafe impl Sync for ZapValueData {}
unsafe impl Sync for ZapValue {}
