use std::{fmt::Display, sync::Arc};

use crate::{Binding, ExprNode, Ident, TypeAST, Visibility};

/// Represents a struct definition in Viper
#[derive(Clone, Debug)]
pub struct StructDef {
    /// The identifier representing the struct
    identifier: Ident,

    /// The fields contained in the struct
    fields: Arc<[Binding]>,

    /// The class methods
    methods: Arc<[StructMethod]>,
}

impl StructDef {
    /// Create a new [StructDef] object
    pub fn new(identifier: Ident, fields: Arc<[Binding]>, methods: Arc<[StructMethod]>) -> StructDef {
        StructDef {
            identifier,
            fields,
            methods,
        }
    }
}

impl Display for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from(format!("struct {} {}", self.identifier, '{'));
        
        for field in self.fields.iter() {
            str += format!("\n    {},", field).as_str();
        }
        for method in self.methods.iter() {
            str += format!("\n{method}").as_str();
        }
        str += "\n}\n";


        write!(f, "{str}")
    }
}

/// Represents a method attached to a structure in Viper
#[derive(Clone, Debug)]
pub struct StructMethod {
    name: Ident,
    parameters: Arc<[Binding]>,
    body: Arc<ExprNode>,
    ret: TypeAST,
    visibility: Visibility,
    is_static: bool,
}

impl StructMethod {
    /// Create a new [StructMethod]
    pub fn new(
        name: Ident, 
        parameters: Arc<[Binding]>, 
        body: Arc<ExprNode>,
        ret: TypeAST,
        visibility: Visibility,
        is_static: bool,
    ) -> StructMethod {
        StructMethod {
            name,
            parameters,
            body,
            ret,
            visibility,
            is_static,
        }
    }
}

impl Display for StructMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        str += match &self.visibility {
            Visibility::Public => "public ",
            Visibility::Private => "private ",
        };

        str += match self.is_static {
            true => "static ",
            false => "method ",
        };

        str += format!("{} (", &self.name).as_str();
        
        for param in self.parameters.iter() {
            str += format!("{param}, ").as_str();
        }
        str += format!("): {} {}\n{}{}\n", self.ret, '{', self.body, '}').as_str();

        write!(f, "{str}")
    }
}
