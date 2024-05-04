use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub name: String,
}

impl Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}()", self.name)
    }
}
