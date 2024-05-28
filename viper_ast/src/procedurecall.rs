use std::fmt::Display;
use crate::ExprNode;

#[derive(Clone, Debug)]
pub struct ProcedureCall {
    name: String,
    arguments: Vec<Box<ExprNode>>,
}

impl ProcedureCall {
    /// Create a new node for a procedure call
    pub fn new(name: String, arguments: Vec<Box<ExprNode>>) -> ProcedureCall {
        ProcedureCall {
            name,
            arguments,
        }
    }
}

impl Display for ProcedureCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from(format!("{}(", self.name));
       
        let mut index = 0;
        for arg in &self.arguments {
            str += format!("{arg}").as_str();
            if index < self.arguments.len()-1 {
                str += ", ";
            }
            index += 1;
        }
        str += ")";

        write!(f, "{str}")
    }
}
