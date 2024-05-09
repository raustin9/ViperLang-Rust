use std::fmt::Display;

use viper_core::token::Token;

use crate::{Ident, TypeAST};

/// Represents binding an identifier to a type
/// let i: i32 = ...
///    |      |
#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    ident: Ident,
    ty: TypeAST,
    // ty: Token,
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

impl Binding {
    pub fn new(ident: String, ty: TypeAST) -> Binding {
        Binding {
            ident,
            ty
        }
    }
}
