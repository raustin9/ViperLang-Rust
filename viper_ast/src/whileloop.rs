use std::fmt::Display;

use crate::ExprNode;

/// Represents a while loop in the Viper programming language
#[derive(Clone, Debug)]
pub struct WhileLoop {
    condition: Box<ExprNode>,

    /// The [CodeBlock] that contains the body
    /// of code to be executed while the condition
    /// holds true
    body: Box<ExprNode>,
}

impl Display for WhileLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} \n{}\n{}{}\n", self.condition, '{', self.body, '}')
    }
}

impl WhileLoop {
    pub fn new(condition: Box<ExprNode>, body: Box<ExprNode>) -> WhileLoop {
        WhileLoop {
            condition,
            body,
        }
    }
}
