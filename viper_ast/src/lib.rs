use std::fmt::Display;
use viper_core::span::Span;

pub mod objinit;
pub use objinit::*;

pub mod structdef;
pub use structdef::*;

pub mod conditional;
pub use conditional::*;

pub mod codeblock;
pub use codeblock::*;

pub mod procedurecall;
pub use procedurecall::*;

pub mod binaryop;
pub use binaryop::*;

pub mod unaryop;
pub use unaryop::*;

pub mod methodcall;
pub use methodcall::*;

pub mod field;
pub use field::*;

pub mod binding;
pub use binding::*;

pub mod variable_init;
pub use variable_init::*;

//pub mod typeast;
//pub use typeast::*;

pub mod whileloop;
pub use whileloop::*;

pub mod proceduredef;
pub use proceduredef::*;

/// Whether something is private or public visible
#[derive(Clone, Debug)]
pub enum Visibility {
    Public,
    Private,
}

/// Represents a node in the Abstract Syntax tree for the Viper programming language
#[derive(Debug, Clone)]
pub struct Node<T> {
    _span: Span,
    inner: T,
}

impl <T> Node<T> {
    /// Create a new AST node
    pub fn new(inner: T, span: Span) -> Node<T> {
        Node {
            inner,
            _span: span,
        }
    }

    /// Accessor to the inner component of the Node
    pub fn inner(&self) -> &T {
        &self.inner
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
pub type Ident = String;

#[derive(Clone, Debug)]
pub enum Expr {
    True,
    False,
    Integer(u64),
    Float(f64),
    StringLiteral(String),
    Return(Box<ExprNode>),
    Yield(Box<ExprNode>),
    ProcedureDefinition(ProcedureDef),
    Let(VariableInitialization),
    WhileLoop(WhileLoop),
    If(Conditional),
    Identifier(String),
    ProcedureCall(Box<ProcedureCall>),
    MethodCall(Box<MethodCall>),
    MemberFieldAccess(Box<Field>),
    BinaryOperation(BinaryOperator, Box<ExprNode>, Box<ExprNode>),
    UnaryOperation(UnaryOperator, Box<ExprNode>),
    CodeBlock(CodeBlock),
    StructDef(StructDef),
    ObjInitialization(ObjInit),
} 

pub type StrType = String;

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
            Self::StringLiteral(literal) => {
                write!(f, "{literal}")
            }
            Self::Return(expr) => {
                write!(f, "return {expr}")
            }
            Self::Yield(expr) => {
                write!(f, "yield {expr}")
            }
            Self::Float(value) => {
                write!(f, "{value}")
            }
            Self::Identifier(name) => {
                write!(f, "{name}")
            }
            Self::If(conditional) => {
                write!(f, "{conditional}")
            }
            Self::WhileLoop(whileloop) => {
                write!(f, "{whileloop}")
            }
            Self::CodeBlock(block) => {
                write!(f, "{}", block)
            }
            Self::ProcedureCall(function) => {
                write!(f, "{}", *function)
            }
            Self::ProcedureDefinition(def) => {
                write!(f, "{}", def)
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
            Self::Let(init) => {
                write!(f, "{}", init)
            }
            Self::UnaryOperation(op, expr) => {
                write!(f, "{}{}", op, expr.inner)
            }
            Self::StructDef(structdef) => {
                write!(f, "{structdef}")
            }
            Self::ObjInitialization(init) => {
                write!(f, "{init}")
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

pub type ExprNode = Node<Expr>;
impl Display for ExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
