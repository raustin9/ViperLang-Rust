use std::{fmt::Display, sync::Arc};

use crate::{source::SourceModule, span::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    module: Arc<SourceModule>,
    name: String,
    span: Span,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(module: Arc<SourceModule>, name: String, span: Span) -> Symbol {
        Symbol {
            module,
            name,
            span,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
