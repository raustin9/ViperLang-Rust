use std::fmt::Display;

use viper_core::token::{PunctuatorKind, Token};


/// The unary operators in the Viper programming language
///
/// Unary operators are ones that only bind to one operand
///
/// eg: !true -> !(true) -> false
/// 
/// eg: -1 -> -(1) -> negative 1
#[derive(Clone, Debug)]
pub enum UnaryOperator {
    /// '-'
    Minus,
    /// '~'
    BinaryNot,
    /// '!'
    LogicalNot,
    /// Not a unary operator
    InvalidUnary,
}

impl From<Token> for UnaryOperator {
    fn from(value: Token) -> Self {
        match value {
            Token::Punctuator(kind, _prec, _span) => {
                match kind {
                    PunctuatorKind::Minus => UnaryOperator::Minus,
                    PunctuatorKind::Bang => UnaryOperator::LogicalNot,
                    PunctuatorKind::Tilde => UnaryOperator::BinaryNot,
                    _ => UnaryOperator::InvalidUnary,
                }
            }
            _ => UnaryOperator::InvalidUnary,
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::BinaryNot=> write!(f, "~"),
            Self::LogicalNot=> write!(f, "!"),
            Self::InvalidUnary=> write!(f, "Invalid Unary Operator"),
        }
    }
}
