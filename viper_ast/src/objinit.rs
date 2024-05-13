use std::{fmt::Display, sync::Arc};

use crate::{ExprNode, Ident};


/// Represents initializing an object in Viper
/// let user = User {
///     name: "Alex",
///     age: 5,
/// };
#[derive(Clone, Debug)]
pub struct ObjInit {
    name: Ident,
    initializations: Vec<FieldInit>,
}

impl ObjInit {
    /// Create a new ObjInit object
    pub fn new(name: Ident, initializations: Vec<FieldInit>) -> ObjInit {
        ObjInit {
            name,
            initializations,
        }
    }
}

impl Display for ObjInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = format!("{} {}\n", self.name, '{');
        for field in &self.initializations {
            str += format!("{field} \n").as_str();
        }
        str += "}";
        write!(f, "{str}")
    }
}

/// Represents a field initialization
/// name: "Alex",
#[derive(Clone, Debug)]
pub struct FieldInit {
    name: Ident,
    value: Arc<ExprNode>,
}

impl FieldInit {
    pub fn new(name: Ident, value: Arc<ExprNode>) -> FieldInit {
        FieldInit {
            name,
            value,
        }
    }
}

impl Display for FieldInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
