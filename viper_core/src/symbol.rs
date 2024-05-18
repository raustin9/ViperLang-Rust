use std::{fmt::Display, sync::Arc};

use crate::{_type::Type, source::SourceModule, span::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    module: Arc<SourceModule>,
    name: String,
    dtype: Arc<Type>,
    span: Span,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(module: Arc<SourceModule>, dtype: Arc<Type>, name: String, span: Span) -> Symbol {
        Symbol {
            module,
            name,
            dtype,
            span,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
