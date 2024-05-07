use std::fmt::Display;

use viper_core::_type::Type;

use crate::Ident;

/// Represents binding an identifier to a type
/// let i: i32 = ...
///    |      |
#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    ident: Ident,
    ty: Type
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}
