use std::{cell::RefCell, fmt::Display, sync::Arc};
use viper_core::scope::Scope;

use crate::{Expr, ExprNode};

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
    scope: Arc<RefCell<Scope>>,
}

impl CodeBlock {
    /// Create a new [CodeBlock] structure.
    pub fn new(exprs: Vec<ExprNode>, scope: Arc<RefCell<Scope>>) -> CodeBlock {
        CodeBlock {
            exprs,
            scope,
        }
    }

    /// Add a parsed expression to the 
    pub fn add_expr(&mut self, expr: ExprNode) {
        match expr.inner() {
            Expr::Let(init) => {
                let sym = init.to_symbol();
                self.scope.as_ref().borrow_mut().add_symbol(init.name(), sym);
            }
            Expr::ProcedureDefinition(def) => {
                self.scope.as_ref().borrow_mut().add_symbol(def.name(), def.to_symbol());
            }
            _ => {
            }
        }
        self.exprs.push(expr);
    }


    /// Return a pointer to the scope of this [CodeBlock]
    pub fn scope(&self) -> Arc<RefCell<Scope>> {
        self.scope.clone()
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Iterate through each expression in the block and print
        // them each out on a line
        let mut str = String::new();

        for expr in &self.exprs {
            str += format!("{};\n", expr).as_str();
        }

        self.scope.as_ref().borrow().print(">> ");

        write!(f, "{str}")
    }
}
