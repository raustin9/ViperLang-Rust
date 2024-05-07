use std::sync::Arc;

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
    Var(i32),
}

/// thanks https://github.com/borgo-lang/borgo/blob/main/compiler/src/type_.rs#L27
#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
    pub generic: Type,
    pub ty: Type,
}
