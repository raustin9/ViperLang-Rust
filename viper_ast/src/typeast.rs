use std::sync::Arc;


/// Represents a type within the Abstract Syntax Tree
/// let i: i32 = ...
/// i32 is a node
#[derive(Clone, Debug, PartialEq)]
pub enum TypeAST {
    Concrete {
        name: String,
        args: Vec<Self>
    },
    Function {
        args: Vec<Self>,
        ret: Arc<Self>
    }
}
