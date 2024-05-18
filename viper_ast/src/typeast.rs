//use std::{fmt::Display, sync::Arc};
//
//
///// Represents a type within the Abstract Syntax Tree
///// let i: i32 = ...
///// i32 is a node
//#[derive(Clone, Debug, PartialEq)]
//pub enum TypeAST {
//    Concrete {
//        name: String,
//
//        /// Arguments to the type. 
//        /// i32, [] etc
//        args: Vec<Self>,
//    },
//    Procedure {
//    }
//}
//
//impl Display for TypeAST {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Self::Concrete { name, args } => {
//                let mut str = String::from(format!("{name}"));
//                for arg in args {
//                    str += format!("{arg}").as_str();
//                }
//
//                write!(f, "{str}")
//            }
//            _ => {
//                write!(f, "FUNCTION TYPEAST TODO")
//            }
//        }
//    }
//}
