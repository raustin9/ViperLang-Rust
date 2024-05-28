use std::{fmt::Display, sync::Arc};

use crate::{_type::Type, source::SourceModule, span::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    module: Arc<SourceModule>,
    name: String,
    dtype: Arc<Type>,
    span: Span,
    is_mutable: bool,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(module: Arc<SourceModule>, dtype: Arc<Type>, name: String, span: Span, is_mutable: bool) -> Symbol {
        Symbol {
            module,
            name,
            dtype,
            span,
            is_mutable,
        }
    }

    pub fn get_key(&self) -> String {
        self.name.clone()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
