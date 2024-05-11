use std::{fmt::Display, sync::Arc};

use crate::{Binding, Ident};

/// Represents a struct definition in Viper
#[derive(Clone, Debug)]
pub struct StructDef {
    /// The identifier representing the struct
    identifier: Ident,

    /// The fields contained in the struct
    fields: Arc<[Binding]>,
}

impl StructDef {
    /// Create a new [StructDef] object
    pub fn new(identifier: Ident, fields: Arc<[Binding]>) -> StructDef {
        StructDef {
            identifier,
            fields,
        }
    }
}

impl Display for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from(format!("struct {} {}", self.identifier, '{'));
        
        for field in self.fields.iter() {
            str += format!("\n    {},", field).as_str();
        }
        str += "\n}";

        write!(f, "{str}")
    }
}
