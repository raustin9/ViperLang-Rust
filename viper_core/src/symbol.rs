use std::sync::Arc;

use crate::{source::SourceModule, span::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    module: Arc<SourceModule>,
    name: String,
    span: Span,
}
