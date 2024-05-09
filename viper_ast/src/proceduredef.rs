use std::{fmt::Display, sync::Arc};

use crate::{Binding, ExprNode, Ident, TypeAST};

/// The variations of procedures that we can have in the Viper programming language
#[derive(Clone, Debug, PartialEq)]
pub enum ProcedureKind {
    /// Top-level procedure that is defined at program or file scope
    /// `
    /// define main(params) {
    ///     ...
    /// }
    /// `
    TopLevel,

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
    parameters: Arc<[Binding]>,
    body: Arc<ExprNode>,
    ret: TypeAST,
    // ret: Token,
}

impl ProcedureDef {
    pub fn new(name: Ident, parameters: Arc<[Binding]>, body: Arc<ExprNode>, ret: TypeAST) -> ProcedureDef {
        ProcedureDef {
            name,
            parameters,
            body,
            ret
        }
    }
}

impl Display for ProcedureDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut procstr = format!("define {}(", self.name);

        let pit = self.parameters.iter();
        for param in pit.as_slice() {
            procstr += format!("{} ", param).as_str();
        }
        procstr += format!("): {} {}\n", self.ret, '{').as_str();

        procstr += format!("{}", self.body).as_str();
        procstr += "}\n";

        write!(f, "{}", procstr)
    }
}
