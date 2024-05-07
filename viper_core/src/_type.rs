use std::{fmt::Display, sync::Arc};

use crate::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Con {
        id: Symbol,
        args: Vec<Type>,
    },
    Proc {
        args: Vec<Type>,
        bound: Vec<Bound>,
        ret: Arc<Type>,
    },
    // Var(i32),
}

/// thanks https://github.com/borgo-lang/borgo/blob/main/compiler/src/type_.rs#L27
#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
    pub generic: Type,
    pub ty: Type,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Con { id, args } => {
                write!(f, "[{id}]")
            }
            Self::Proc { args, bound, ret } => {
                let mut typestr = String::from("[fn ");
                for arg in args {
                    typestr += format!("{arg} ").as_str()
                }

                typestr += format!("{ret}").as_str();

                write!(f, "")
            }
        }
    }
}
