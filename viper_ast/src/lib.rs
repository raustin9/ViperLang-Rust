use std::sync::Arc;
use viper_core::span::Span;


pub mod functioncall;
pub use functioncall::*;

pub mod binaryop;
pub use binaryop::*;

pub mod unaryop;
pub use unaryop::*;

pub mod methodcall;
pub use methodcall::*;

pub mod field;
pub use field::*;

pub mod variable_init;
pub use variable_init::*;

#[derive(Debug)]
pub struct AST {
}


/// Represents a node in the Abstract Syntax tree for the Viper programming language
#[derive(Debug, Clone)]
pub struct Node<T> {
    span: Span,
    inner: T,
}

impl <T> Node<T> {
    /// Create a new AST node
    pub fn new(inner: T, span: Span) -> Node<T> {
        Node {
            inner,
            span
        }
    }
}

pub enum Stmt {
    VariableInitialization(VariableInitialization),
    FunctionDefinition,
    Conditional,
    WhileLoop,
    DoWhileLoop,
    ForLoop,
    ExpressionStatement,
}

/// Represents expression types in Viper
///
/// Expressions are pieces of code that hold [or evaluate to] values
///
/// eg: 5 + 1 is an expression because it evaluates to 6
/// 
/// NOTE: Literals are also expressions that evaluate to themselves
/// 
/// eg: 5 evaluates to 5, "test" evaluates to "test"
#[derive(Clone, Debug)]
pub enum Expr {
    True,
    False,
    Integer(u64),
    Float(f64),
    Identifier(String),
    FunctionCall(Arc<FunctionCall>),
    MethodCall(Arc<MethodCall>),
    MemberFieldAccess(Arc<Field>),
    BinaryOperation(BinaryOperator, Arc<ExprNode>, Arc<ExprNode>),
    UnaryOperation(UnaryOperator, Arc<ExprNode>),
}

/// The floating point data types
pub enum FloatType {
    /// 32 bit floating point number IEEE754
    Float32,
    
    /// 64 bit floating point number IEEE754
    Float64
}

/// Integer data types
pub enum IntegerType {
    /// 8 bit signed integer
    Int8,
    /// 16 bit signed integer
    Int16,
    /// 32 bit signed integer
    Int32,
    /// 64 bit signed integer
    Int64,
    /// 8 bit unsigned integer
    UInt8,
    /// 16 bit unsigned integer
    UInt16,
    /// 32 bit unsigned integer
    UInt32,
    /// 64 bit unsigned integer
    UInt64,
}

pub type StmtNode = Node<Stmt>;
pub type ExprNode = Node<Expr>;
