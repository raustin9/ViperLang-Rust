use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct ProcedureCall {
    name: String,
}

impl Display for ProcedureCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}()", self.name)
    }
}
