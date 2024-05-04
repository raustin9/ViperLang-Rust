use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct FunctionCall {
    name: String,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}()", self.name)
    }
}
