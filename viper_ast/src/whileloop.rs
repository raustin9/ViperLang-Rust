use std::sync::Arc;

use crate::{CodeBlock, ExprNode};

/// Represents a while loop in the Viper programming language
#[derive(Clone, Debug)]
pub struct WhileLoop {
    condition: Arc<ExprNode>,
    body: CodeBlock,
}

impl WhileLoop {
    pub fn new(condition: Arc<ExprNode>, body: CodeBlock) -> WhileLoop {
        WhileLoop {
            condition,
            body,
        }
    }
}
