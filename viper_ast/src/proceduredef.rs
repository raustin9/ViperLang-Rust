use std::fmt::Display;

use viper_core::_type::Type;

use crate::{Binding, ExprNode, Ident};

/// The variations of procedures that we can have in the Viper programming language
#[derive(Clone, Debug, PartialEq)]
pub enum ProcedureKind {
    /// Top-level procedure that is defined at program or file scope
    /// `
    /// proc main(params) {
    ///     ...
    /// }
    /// `
    TopLevel,
   
    /// Functions defined within another function
    /// `
    /// proc main() {
    ///     proc test() {
    ///         return 6;
    ///     }
    ///     ...
    /// }
    /// `
    Inline,

    /// Lambda functions
    /// `
    /// let lambda = |a, b|: i32 => {
    ///     return a + b;
    /// }
    /// `
    Lambda,
}

#[derive(Clone, Debug)]
pub struct ProcedureDef {
    name: Ident,
    parameters: Vec<Binding>,
    statements: Vec<ExprNode>,
    ty: Type,
    ret: Type,
}

impl Display for ProcedureDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut procstr = format!("proc {}(", self.name);
        let mut body = String::from("");

        for param in &self.parameters {
            procstr += format!("{} ", param).as_str();
        }
        procstr += format!("): {} {}\n", self.ret, '{').as_str();
        
        for stmt in &self.statements {
            procstr += format!("{}\n", stmt).as_str();
        }
        procstr += "{\n";

        write!(f, "{}", procstr)
    }
}
