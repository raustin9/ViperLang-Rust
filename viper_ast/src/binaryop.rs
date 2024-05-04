use std::fmt::Display;

use viper_core::token::{PunctuatorKind, Token};

/// The valid operators for binary expressions in the Viper programming language
///
/// Binary expressions have two operands, a left and right, and 1 operator that binds to them
///
/// eg: 1 + 2
#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    /// '-'
    Minus,
    /// '+'
    Plus,
    
    /// '*'
    Times,
    /// '/'
    Divide,
    /// '%'
    Modulo,

    /// '<'
    LessThan,
    /// '>'
    GreaterThan,
    /// '=='
    EqualTo,
    /// '!='
    NotEqualTo,
    /// '<='
    LessThanEqualTo,
    /// '>='
    GreaterThanEqualTo,

    /// '&&'
    LogicalAnd,
    /// '||'
    LogicalOr,
   
    /// '>>'
    BShiftRight,
    /// '<<'
    BShiftLeft,
    /// '&'
    BinAnd,
    /// '|'
    BinOr,
    /// '^'
    BinXor,

    /// Invalid Binary Operator
    InvalidBinary,
}

impl From<Token> for BinaryOperator {
    fn from(value: Token) -> Self {
        match value {
            Token::Punctuator(kind, _, _) => {
                match kind {
                    PunctuatorKind::Plus => BinaryOperator::Plus,
                    PunctuatorKind::Minus => BinaryOperator::Minus,
                    PunctuatorKind::Star => BinaryOperator::Times,
                    PunctuatorKind::Slash => BinaryOperator::Divide,
                    PunctuatorKind::Mod => BinaryOperator::Modulo,
                    PunctuatorKind::Ampersand => BinaryOperator::BinAnd,
                    PunctuatorKind::Pipe => BinaryOperator::BinOr,
                    PunctuatorKind::Caret => BinaryOperator::BinXor,
                    PunctuatorKind::LShift => BinaryOperator::BShiftLeft,
                    PunctuatorKind::RShift => BinaryOperator::BShiftRight,
                    
                    PunctuatorKind::EqualTo => BinaryOperator::EqualTo,
                    PunctuatorKind::NotEqualTo => BinaryOperator::NotEqualTo,
                    PunctuatorKind::LessThan => BinaryOperator::LessThan,
                    PunctuatorKind::LessThanEQ => BinaryOperator::LessThanEqualTo,
                    PunctuatorKind::GreaterThan => BinaryOperator::GreaterThan,
                    PunctuatorKind::GreaterThanEQ => BinaryOperator::GreaterThanEqualTo,
                    PunctuatorKind::LogicalOr => BinaryOperator::LogicalOr,
                    PunctuatorKind::LogicalAnd => BinaryOperator::LogicalAnd,
                    _ => BinaryOperator::InvalidBinary,
                }
            }
            _ => BinaryOperator::InvalidBinary,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minus => {
                write!(f, "-")
            }
            Self::Plus => {
                write!(f, "+")
            }
            Self::Times => {
                write!(f, "*")
            }
            Self::Divide => {
                write!(f, "/")
            }
            Self::Modulo => {
                write!(f, "%")
            }
            Self::LessThan => {
                write!(f, "<")
            }
            Self::GreaterThan => {
                write!(f, ">")
            }
            Self::EqualTo => {
                write!(f, "==")
            }
            Self::NotEqualTo => {
                write!(f, "!=")
            }
            Self::LessThanEqualTo => {
                write!(f, "<=")
            }
            Self::GreaterThanEqualTo => {
                write!(f, ">=")
            }
            Self::LogicalAnd => {
                write!(f, "&&")
            }
            Self::LogicalOr => {
                write!(f, "||")
            }
            Self::BShiftRight => {
                write!(f, ">>")
            }
            Self::BShiftLeft => {
                write!(f, "<<")
            }
            Self::BinAnd => {
                write!(f, "&")
            }
            Self::BinOr => {
                write!(f, "|")
            }
            Self::BinXor => {
                write!(f, "^")
            }
            Self::InvalidBinary => {
                write!(f, "Invalid Binary Operator")
            }
        }
    }
}
