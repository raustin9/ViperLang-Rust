/// Abstract Syntax Tree for the Viper programming language
///
/// The [`AST`] represents a Viper program  from recursive 
/// data types that form a tree structure with the [`Program`]
/// type as the root.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct AST {
    pub root: Program,
}
