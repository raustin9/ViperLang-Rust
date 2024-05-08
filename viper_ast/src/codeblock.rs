use std::{fmt::Display, sync::Arc};
use viper_core::scope::Scope;

use crate::{ExprNode};

/// Represents a block of code in the Viper programming language
/// `
/// {
///     let i: i32 = 0;
///     foo();
/// }
/// `
#[derive(Clone, Debug)]
pub struct CodeBlock {
    /// Vector of the expressions contained within
    /// this block of code
    exprs: Vec<ExprNode>,

    /// The scope that contains this block of code
    scope: Arc<Scope>,
}

impl CodeBlock {
    /// Create a new [CodeBlock] structure.
    pub fn new(parent: Option<Arc<Scope>>) -> CodeBlock {
        CodeBlock {
            exprs: Vec::new(),
            scope: Arc::from(Scope::new(parent)),
        }
    }

    /// Add a parsed expression to the 
    pub fn add_expr(&mut self, expr: ExprNode) {
        self.exprs.push(expr);
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Iterate through each expression in the block and print
        // them each out on a line
        let mut str = String::new();

        for expr in &self.exprs {
            str += format!("{}\n", expr).as_str();
        }

        write!(f, "{str}")
    }
}
