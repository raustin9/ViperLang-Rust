use std::{fmt::Display, sync::Arc};
use viper_core::{_type::Type, span::Span};

pub mod scope;
pub use scope::*;

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

pub mod typeast;
pub use typeast::*;

pub mod whileloop;
pub use whileloop::*;

pub mod proceduredef;
pub use proceduredef::*;

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

//#[derive(Clone, Debug)]
//pub enum Expr {
//    Literal {
//        literal: Literal,
//        ty: Type,
//        span: Span,
//    },
//    Closure {
//        proc: ProcedureCall,
//        kind: ProcedureKind,
//        ty: Type,
//        span: Span,
//    },
//    Block {
//        statements: Vec<Expr>,
//        ty: Type,
//        span: Span,
//    },
//    Let {
//        binding: Binding,
//        ident: Vec<Expr>,
//        value: Arc<Expr>,
//        types: Vec<Type>,
//        span: Span,
//    },
//    Variable {
//        value: Ident,
//        decl: Span,
//        // TODO: generics
//        ty: Type,
//        span: Span,
//    },
//    ProcedureCall {
//        proc: Arc<Expr>,
//        args: Vec<Expr>,
//        ty: Type,
//        span: Span,
//    },
//    If {
//        cond: Arc<Expr>,
//        then: Arc<Expr>,
//        ty: Type,
//        span: Span,
//    },
//    Match {
//        subject: Arc<Expr>,
//        arms: TypeAST,
//        ty: Type,
//        span: Span,
//    },
//    Tuple {
//        elements: Vec<Expr>,
//        ty: Type,
//        span: Span,
//    },
//    EnumDefinition {
//        // TODO: Enums
//    },
//}

#[derive(Clone, Debug)]
pub enum Expr {
    True,
    False,
    Integer(u64),
    Float(f64),
    Return(Arc<ExprNode>),
    ProcedureDefinition(ProcedureDef),
    Let(VariableInitialization),
    WhileLoop(WhileLoop),
    Identifier(String),
    ProcedureCall(Arc<ProcedureCall>),
    MethodCall(Arc<MethodCall>),
    MemberFieldAccess(Arc<Field>),
    BinaryOperation(BinaryOperator, Arc<ExprNode>, Arc<ExprNode>),
    UnaryOperation(UnaryOperator, Arc<ExprNode>),
    CodeBlock(CodeBlock),
} 

/// Represents a literal type in Viper
/// 1, true, false, "string literal", 'c', [1, 3, 5]
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(u64),
    Float(f64),
    Bool(bool),
    String(StrType),
    Char(char),
    Slice, // TODO
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
            Self::Return(expr) => {
                write!(f, "return {expr}")
            }
            Self::Float(value) => {
                write!(f, "{value}")
            }
            Self::Identifier(name) => {
                write!(f, "{name}")
            }
            Self::WhileLoop(_loop_info) => {
                todo!()
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
