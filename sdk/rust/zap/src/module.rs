use crate::Function;

pub struct Module {
    functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn function(mut self, function: Function) -> Self {
        self.functions.push(function);
        self
    }
}
