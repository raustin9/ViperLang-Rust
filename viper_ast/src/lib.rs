use std::{fmt::Display, sync::Arc};
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

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VariableInitialization(init) => write!(f, "{init}"),
            _ => write!(f, "Not handled yet"),
        }
    }
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::True => {
                write!(f, "true")
            }
            Self::False => {
                write!(f, "false")
            }
            Self::Integer(value) => {
                write!(f, "{value}")
            }
            Self::Float(value) => {
                write!(f, "{value}")
            }
            Self::Identifier(name) => {
                write!(f, "{name}")
            }
            Self::FunctionCall(function) => {
                write!(f, "{}", *function)
            }
            Self::MethodCall(method) => {
                write!(f, "{}", *method)
            }
            Self::MemberFieldAccess(field) => {
                write!(f, "{}", *field)
            }
            Self::BinaryOperation(op, lhs, rhs) => {
                write!(f, "[{} {} {}]", lhs.inner, op, rhs.inner)
            }
            Self::UnaryOperation(op, expr) => {
                write!(f, "{}{}", op, expr.inner)
            }
        }
    }
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
impl Display for StmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

pub type ExprNode = Node<Expr>;
impl Display for ExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
