use std::{fmt::Display, sync::Arc};
use viper_core::token::Token;

use crate::ExprNode;

/// ## AST Node structure for initializing a variable
///
/// `let i: i32 = 0;`
/// -> Declare variable `i` of type `i32` and init it to 0.
///
/// `let i, j: i32 = 0;`
///  -> Declare two variables i & j and initialize them to 0.
///
/// `let i, j, k: i32 = 1, 2, 4;` 
/// -> Declare i j and k and init them to 1, 2 and 4 respectively.
#[derive(Clone, Debug)]
pub struct VariableInitialization {
    targets: Vec<Arc<ExprNode>>,
    dtype: Token,
    values: Vec<Arc<ExprNode>>,
}

impl VariableInitialization {
    /// Create a new VariableInitialization
    pub fn new(targets: Vec<Arc<ExprNode>>, dtype: Token, values: Vec<Arc<ExprNode>>) -> VariableInitialization {
        VariableInitialization {
            targets,
            dtype,
            values,
        }
    }
}

impl Display for VariableInitialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {}: {} = {}", 
            self.targets[0].inner,
            self.dtype,
            self.values[0].inner
        )
    }
}
