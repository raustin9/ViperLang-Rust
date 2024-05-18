use std::{collections::HashMap, sync::Arc};

use crate::symbol::Symbol;


/// Represents a scope 
#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    /// Symbol table for all symbols defined within this scope
    symbol_map: HashMap<String, Symbol>,
   
    /// Pointer to the parent [Scope] that contains this scope
    /// None if there is not a parent scope
    parent: Option<Arc<Scope>>,
}

impl Scope {
    /// Create a new [Scope] object with the parent specified
    pub fn new(parent: Option<Arc<Scope>>) -> Scope {
        Scope {
            symbol_map: HashMap::new(),
            parent
        }
    }

    /// Insert a symbol into the Scope at the given key
    pub fn add_symbol(&mut self, key: String, symbol: Symbol) {
        self.symbol_map.insert(key, symbol);
    }
}
