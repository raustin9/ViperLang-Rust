use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::symbol::Symbol;


/// Represents a scope 
#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    /// Symbol table for all symbols defined within this scope
    symbol_map: HashMap<String, Symbol>,
   
    /// Pointer to the parent [Scope] that contains this scope
    /// None if there is not a parent scope
    parent: Option<Arc<RefCell<Scope>>>,
}

impl Scope {
    /// Create a new [Scope] object with the parent specified
    pub fn new(parent: Option<Arc<RefCell<Scope>>>) -> Scope {
        Scope {
            symbol_map: HashMap::new(),
            parent
        }
    }

    /// Insert a symbol into the Scope at the given key
    pub fn add_symbol(&mut self, key: String, symbol: Symbol) {
        self.symbol_map.insert(key, symbol);
    }

    /// Print the values in this table and all tables within this one
    pub fn print(&self, prefix: &str) {
        println!("Symbol Table:");
        for (ref key, ref sym) in &self.symbol_map {
            println!("{prefix}{key}: {sym}");
        }
        println!("End Table.");
    }
}
