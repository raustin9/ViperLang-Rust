use std::{fmt::Display, sync::Arc};


/// Represents a type within the Abstract Syntax Tree
/// let i: i32 = ...
/// i32 is a node
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Concrete {
        name: String,

        /// Arguments to the type. 
        /// i32, [] etc
        args: Vec<Self>,
    },
    Procedure {
        /// Name of the procedure
        name: String,

        params: Vec<Self>,

        /// Return type of the procedure
        return_type: Arc<Self>,
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concrete { name, args } => {
                let mut str = String::from(format!("{name}"));
                for arg in args {
                    str += format!("{arg}").as_str();
                }

                write!(f, "{str}")
            }
            Self::Procedure { name, ref params, return_type } => {
                let mut str = format!("Proc_{name}");

                for p in params {
                    str += format!("_{p}").as_ref();
                }

                str += format!("_{return_type}").as_ref();

                write!(f, "{str}")
            }
        }
    }
}

