use crate::*;

pub struct Function {
    pub name: &'static str,
    pub arity: usize,
    pub callback: fn(Vec<Value>) -> Result<Value>,
}
