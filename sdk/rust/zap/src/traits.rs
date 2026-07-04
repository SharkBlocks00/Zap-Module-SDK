use zap_sdk::ZapValue;

use crate::Result;

/// Convert a Zap ABI value into a Rust value.
pub trait FromZapValue: Sized {
    /// Attempts to convert a Zap value into this Rust type.
    fn from_zap(value: &ZapValue) -> Result<Self>;
}

/// Convert a Rust value into a Zap ABI value.
pub trait IntoZapValue {
    /// Converts this Rust value into a Zap value.
    fn into_zap(self) -> ZapValue;
}

impl IntoZapValue for () {
    fn into_zap(self) -> ZapValue {
        ZapValue::null()
    }
}

impl FromZapValue for () {
    fn from_zap(_: &ZapValue) -> Result<Self> {
        Ok(())
    }
}

impl IntoZapValue for bool {
    fn into_zap(self) -> ZapValue {
        ZapValue::boolean(self)
    }
}

impl FromZapValue for bool {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        crate::convert::bool_from_zap(value)
    }
}
impl IntoZapValue for i64 {
    fn into_zap(self) -> ZapValue {
        ZapValue::int(self)
    }
}

impl FromZapValue for i64 {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        crate::convert::i64_from_zap(value)
    }
}
impl IntoZapValue for i32 {
    fn into_zap(self) -> ZapValue {
        ZapValue::int(self as i64)
    }
}

impl FromZapValue for i32 {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        Ok(crate::convert::i64_from_zap(value)? as i32)
    }
}
impl IntoZapValue for usize {
    fn into_zap(self) -> ZapValue {
        ZapValue::int(self as i64)
    }
}

impl FromZapValue for usize {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        Ok(crate::convert::i64_from_zap(value)? as usize)
    }
}
impl IntoZapValue for u64 {
    fn into_zap(self) -> ZapValue {
        ZapValue::int(self as i64)
    }
}

impl FromZapValue for u64 {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        Ok(crate::convert::i64_from_zap(value)? as u64)
    }
}
impl IntoZapValue for f64 {
    fn into_zap(self) -> ZapValue {
        ZapValue::float(self)
    }
}

impl FromZapValue for f64 {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        crate::convert::f64_from_zap(value)
    }
}
impl IntoZapValue for f32 {
    fn into_zap(self) -> ZapValue {
        ZapValue::float(self as f64)
    }
}

impl FromZapValue for f32 {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        Ok(crate::convert::f64_from_zap(value)? as f32)
    }
}
impl IntoZapValue for String {
    fn into_zap(self) -> ZapValue {
        ZapValue::owned_string(self)
    }
}

impl FromZapValue for String {
    fn from_zap(value: &ZapValue) -> Result<Self> {
        crate::convert::string_from_zap(value)
    }
}
impl IntoZapValue for &str {
    fn into_zap(self) -> ZapValue {
        ZapValue::owned_string(self)
    }
}

impl<T> IntoZapValue for Vec<T>
where
    T: IntoZapValue,
{
    fn into_zap(self) -> ZapValue {
        todo!("Native array support is coming in ABI v2.")
    }
}

impl<T> FromZapValue for Vec<T>
where
    T: FromZapValue,
{
    fn from_zap(_: &ZapValue) -> Result<Self> {
        todo!("Native array support is coming in ABI v2.")
    }
}

impl<T> IntoZapValue for Option<T>
where
    T: IntoZapValue,
{
    fn into_zap(self) -> ZapValue {
        match self {
            Some(v) => v.into_zap(),
            None => ZapValue::null(),
        }
    }
}

impl<T> FromZapValue for Option<T>
where
    T: FromZapValue,
{
    fn from_zap(value: &ZapValue) -> Result<Self> {
        if value.r#type == zap_sdk::TYPE_NULL {
            Ok(None)
        } else {
            Ok(Some(T::from_zap(value)?))
        }
    }
}
