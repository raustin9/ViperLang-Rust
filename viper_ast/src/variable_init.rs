use std::{fmt::Display, sync::Arc};

use viper_core::{_type::Type, source::SourceModule, span::Span, symbol::Symbol};

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
    targets: Vec<Box<ExprNode>>,
    dtype: Type,
    // dtype: Token,
    mutable: bool,
    values: Vec<Box<ExprNode>>,
}

impl VariableInitialization {
    /// Create a new VariableInitialization
    pub fn new(targets: Vec<Box<ExprNode>>, dtype: Type, mutable: bool, values: Vec<Box<ExprNode>>) -> VariableInitialization {
        VariableInitialization {
            targets,
            dtype,
            mutable,
            values,
        }
    }

    /// Create and return a symbol to insert into a symbol table
    /// from the information in this declaration
    pub fn to_symbol(&self) -> Symbol {
        Symbol::new(
            Arc::from(SourceModule::new_dummy()),
            Arc::from(self.dtype.clone()), 
            self.targets[0].to_string(), 
            Span::dummy(), 
            self.mutable
        )
    }

    pub fn name(&self) -> String {
        self.targets[0].to_string().clone()
    }
}

impl Display for VariableInitialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} {}: {} = {}", 
            if self.mutable {"mut"} else {""},
            self.targets[0].inner,
            self.dtype,
            self.values[0].inner
        )
    }
}
