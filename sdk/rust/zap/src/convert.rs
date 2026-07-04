//! Conversion utils for Rust values to/from Zap values

use std::ffi::CStr;

use zap_sdk::{
    ZapValue, TYPE_ARRAY, TYPE_BOOL, TYPE_FLOAT, TYPE_FUNCTION, TYPE_INT, TYPE_NULL, TYPE_OBJECT,
    TYPE_STRING,
};

use crate::{Error, Result, Value};

/// Converts a rust value into abi value
pub trait IntoZapValue {
    fn into_zap(self) -> ZapValue;
}

/// Converts an abi value into a rust vale
pub trait FromZapValue: Sized {
    fn from_zap(value: ZapValue) -> Result<Self>;
}

// Generic value

impl IntoZapValue for Value {
    fn into_zap(self) -> ZapValue {
        match self {
            Value::Null => ZapValue::null(),
            Value::Bool(v) => ZapValue::boolean(v),
            Value::Int(v) => ZapValue::int(v),
            Value::Float(v) => ZapValue::float(v),
            Value::String(v) => ZapValue::owned_string(v),

            Value::Array(_) => {
                todo!("Native arrays are not implemented yet.")
            }
            Value::Object(_) => {
                todo!("Native objects are not implemented yet.")
            }
        }
    }
}

impl FromZapValue for Value {
    fn from_zap(value: ZapValue) -> Result<Self> {
        unsafe {
            Ok(match value.ty {
                TYPE_NULL => Value::Null,

                TYPE_BOOL => Value::Bool(value.data.boolean),

                TYPE_INT => Value::Int(value.data.integer),

                TYPE_FLOAT => Value::Float(value.data.floating),

                TYPE_STRING => {
                    let ptr = value.data.string.data;

                    if ptr.is_null() {
                        return Err(Error::NullPointer);
                    }

                    let s = CStr::from_ptr(ptr)
                        .to_str()
                        .map_err(|_| Error::InvalidUtf8)?;

                    Value::String(s.to_owned())
                }

                TYPE_ARRAY => {
                    todo!("Native arrays are not implemented yet.")
                }

                TYPE_OBJECT => {
                    todo!("Native objects are not implemented yet.")
                }

                TYPE_FUNCTION => {
                    return Err(Error::InvalidType);
                }

                _ => {
                    return Err(Error::InvalidType);
                }
            })
        }
    }
}

// IntoZapValue implementations
impl IntoZapValue for () {
    fn into_zap(self) -> ZapValue {
        ZapValue::null()
    }
}

impl IntoZapValue for bool {
    fn into_zap(self) -> ZapValue {
        ZapValue::boolean(self)
    }
}

macro_rules! impl_int_into {
    ($($t:ty),*) => {
        $(
        impl IntoZapValue for $t {
            fn into_zap(self) -> ZapValue {
                ZapValue::int(self as i64)
            }
        }
        )*
    };
}

impl_int_into!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

macro_rules! impl_float_into {
    ($($t:ty),*) => {
        $(
        impl IntoZapValue for $t {
            fn into_zap(self) -> ZapValue {
                ZapValue::float(self as f64)
            }
        }
        )*
    };
}

impl_float_into!(f32, f64);

impl IntoZapValue for String {
    fn into_zap(self) -> ZapValue {
        ZapValue::owned_string(self)
    }
}

impl IntoZapValue for &str {
    fn into_zap(self) -> ZapValue {
        ZapValue::owned_string(self)
    }
}

impl<T: IntoZapValue> IntoZapValue for Option<T> {
    fn into_zap(self) -> ZapValue {
        match self {
            Some(v) => v.into_zap(),
            None => ZapValue::null(),
        }
    }
}

// FromZapValue implementations

impl FromZapValue for bool {
    fn from_zap(value: ZapValue) -> Result<Self> {
        match Value::from_zap(value)? {
            Value::Bool(v) => Ok(v),
            _ => Err(Error::InvalidType),
        }
    }
}

macro_rules! impl_int_from {
    ($($t:ty),*) => {
        $(
            impl FromZapValue for $t {
                fn from_zap(value: ZapValue) -> Result<Self> {
                    match Value::from_zap(value)? {

                        Value::Int(v) => Ok(v as $t),

                        _ => Err(Error::InvalidType),
                    }
                }
            }
        )*
    };
}

impl_int_from!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

macro_rules! impl_float_from {
    ($($t:ty),*) => {
        $(
            impl FromZapValue for $t {
                fn from_zap(value: ZapValue) -> Result<Self> {

                    match Value::from_zap(value)? {

                        Value::Float(v) => Ok(v as $t),

                        Value::Int(v) => Ok(v as $t),

                        _ => Err(Error::InvalidType),
                    }
                }
            }
        )*
    };
}

impl_float_from!(f32, f64);

impl FromZapValue for String {
    fn from_zap(value: ZapValue) -> Result<Self> {
        match Value::from_zap(value)? {
            Value::String(v) => Ok(v),

            _ => Err(Error::InvalidType),
        }
    }
}

impl<T: FromZapValue> FromZapValue for Option<T> {
    fn from_zap(value: ZapValue) -> Result<Self> {
        if value.ty == TYPE_NULL {
            return Ok(None);
        }

        Ok(Some(T::from_zap(value)?))
    }
}
