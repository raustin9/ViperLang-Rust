use std::{fmt::Display, sync::Arc};

use viper_core::_type::Type;

use crate::{Binding, ExprNode, Ident};

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

/// Represents the definition of a procedure in Viper
/// 
/// define main(argc: i32, argv: u8[][]): i32 {
///     ...
///     return 0;
/// }
/// 
#[derive(Clone, Debug)]
pub struct ProcedureDef {
    /// The name of the procedure
    name: Ident,

    /// The parameter list of what needs to be passed to the procedure
    parameters: Arc<[Binding]>,

    /// The code body of the procedure
    body: Arc<ExprNode>,

    /// The return type of the procedure
    ret: Type,
}

impl ProcedureDef {
    /// Create a new [ProcedureDef] object
    pub fn new(name: Ident, parameters: Arc<[Binding]>, body: Arc<ExprNode>, ret: Type) -> ProcedureDef {
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
            procstr += format!("{}, ", param).as_str();
        }
        procstr += format!("): {} {}\n", self.ret, '{').as_str();

        procstr += format!("{}", self.body).as_str();
        procstr += "}\n";

        write!(f, "{}", procstr)
    }
}
