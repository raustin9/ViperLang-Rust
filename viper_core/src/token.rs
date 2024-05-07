use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use crate::span::Span;

/// The types of tokens that are valid input in the Viper programming language
#[derive(Clone, Debug)]
pub enum Token {
    Keyword(KeywordKind, Span),
    Punctuator(PunctuatorKind, Option<OperatorPrecedence>, Span),
    NumericLiteral(NumericValue, Span),
    StringLiteral(String, Span),
    Identifier(String, Span),
    Illegal(String, Span),
    EOF,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Keyword(_, _) => {
                match other {
                    Self::Keyword(_, _) => true,
                    _ => false,
                }
            }
            Self::NumericLiteral(_, _) => {
                match other {
                    Self::NumericLiteral(_, _) => true,
                    _ => false,
                }
            }
            Self::StringLiteral(_, _) => {
                match other {
                    Self::StringLiteral(_, _) => true,
                    _ => false,
                }
            }
            Self::Illegal(_, _) => {
                match other {
                    Self::Illegal(_, _) => true,
                    _ => false,
                }
            }
            Self::Identifier(_, _) => {
                match other {
                    Self::Identifier(_, _) => true,
                    _ => false,
                }
            }
            Self::Punctuator(_, _, _) => {
                match other {
                    Self::Punctuator(_, _, _) => true,
                    _ => false,
                }
            }
            Self::EOF => {
                match other {
                    Self::EOF => true,
                    _ => false,
                }
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, fout: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(keyword, _span) => {
                write!(fout, "{}", keyword.as_str())
            }
            Self::Punctuator(kind, precedence, _span) => {
                match precedence {
                    Some(prec) => write!(fout, "{} -> Prec: {}", kind.as_str(), prec.as_str()),
                    None => write!(fout, "{} -> Prec: None", kind.as_str()),
                }
                            }
            Self::NumericLiteral(value, _span) => {
                write!(fout, "{}", value)                    
            }
            Self::StringLiteral(string_literal, _span) => {
                write!(fout, "{}", string_literal)
            }
            Self::Identifier(literal, _span) => {
                write!(fout, "Identifier: '{}'", literal)
            }
            Self::Illegal(msg, _span) => {
                write!(fout, "Illegal token '{msg}'")
            }
            Self::EOF => {
                write!(fout, "EOF")
            }
        }
    }
}

/// Enumeration of the types of keywords that are available in the Viper programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum KeywordKind {
    /// Declarator keywords
    Define,
    Proc,
    Let,
    Mut,
    Return,
    Match,
    Struct,
    Enum,
    Module,
    Public,
    Import,
    Export,
   
    /// Control flow keywords
    For,
    While,
    Do,
    If,
    Elif,
    Else,
    Switch,
    Continue,
    Break,
    Case,
    Default,
  
    /// Type Definition Keywords
    Byte,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

impl KeywordKind {
    /// Get the string representation of each Keyword type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Define => return "define",
            Self::Proc => return "proc",
            Self::Let => return "let",
            Self::Mut => return "mut",
            Self::Return => return "return",
            Self::Match => return "match",
            Self::Struct => return "struct",
            Self::Enum => return "enum",
            Self::Module => return "module",
            Self::Public => return "public",
            Self::Import => return "import",
            Self::Export => return "export",
            Self::For => return "for",
            Self::While => return "while",
            Self::Do => return "do",
            Self::If => return "if",
            Self::Elif => return "elif",
            Self::Else => return "else",
            Self::Switch => return "switch",
            Self::Continue => return "continue",
            Self::Break => return "break",
            Self::Case => return "case",
            Self::Default => return "default",
            Self::Byte => return "byte",
            Self::F32 => return "f32",
            Self::F64 => return "f64",
            Self::I8 => return "i8",
            Self::I16 => return "i16",
            Self::I32 => return "i32",
            Self::I64 => return "i64",
            Self::U8 => return "u8",
            Self::U16 => return "u16",
            Self::U32 => return "u32",
            Self::U64 => return "u64",
        }
    }
}


/// Operator precedences for binding expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, Default)]
pub enum OperatorPrecedence {
    #[default]
    Lowest = 0,
    // Assign,
    LogicalAndOr,
    Comparison,
    AddSub,
    MulDivMod,
    Bitshift,
    Prefix,
}


/// Enumeration of the types of punctuators that are available in the Viper programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum PunctuatorKind {
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Bang,
    Tilde,
    Caret,
    Ampersand,
    Pipe,

    LShift,
    RShift,

    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanEQ,
    GreaterThanEQ,

    LogicalAnd,
    LogicalOr,

    EqualSign,

    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivEquals,
    ModEquals,
    RShiftEquals,
    LShiftEquals,
    BinNotEquals,
    BinOrEquals,
    BinXorEquals,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LSquirly,
    RSquirly,
   
    // Typical puncuation
    Comma,
    Dot,
    Colon,
    DoubleColon,
    SemiColon,
    FatArrow,
}

impl PunctuatorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plus => return "+",
            Self::Minus => return "-",
            Self::Star => return "*",
            Self::Slash => return "/",
            Self::Mod => return "%",
            Self::Bang => return "!",
            Self::Tilde => return "~",
            Self::Caret => return "^",
            Self::Ampersand => return "&",
            Self::Pipe => return "|",
            Self::LShift => return "<<",
            Self::RShift => return ">>",
            Self::EqualTo => return "==",
            Self::NotEqualTo => return "!=",
            Self::LessThan => return "<",
            Self::GreaterThan => return ">",
            Self::LessThanEQ => return "<=",
            Self::GreaterThanEQ => return ">=",
            Self::LogicalAnd => return "&&",
            Self::LogicalOr => return "||",
            Self::EqualSign => return "=",
            Self::PlusEquals => return "+=",
            Self::MinusEquals => return "-=",
            Self::TimesEquals => return "*=",
            Self::DivEquals => return "/=",
            Self::ModEquals => return "%=",
            Self::RShiftEquals => return ">>=",
            Self::LShiftEquals => return "<<=",
            Self::BinNotEquals => return "~=",
            Self::BinOrEquals => return "|=",
            Self::BinXorEquals => return "^=",
            Self::LParen => return "(",
            Self::RParen => return ")",
            Self::LBrace => return "[",
            Self::RBrace => return "]",
            Self::LSquirly => return "{",
            Self::RSquirly => return "}",
            Self::Comma => return ",",
            Self::Dot => return ".",
            Self::Colon => return ":",
            Self::DoubleColon => return "::",
            Self::SemiColon => return ";",
            Self::FatArrow => return "=>",
        }
    }
}


/// Implementation for operator precedences
impl OperatorPrecedence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lowest => return "Lowest",
            // Self::Assign => return "Assign",
            Self::LogicalAndOr => return "LogicalAndOr",
            Self::Comparison => return "Comparison",
            Self::AddSub => return "AddSub",
            Self::MulDivMod => return "MulDivMod",
            Self::Bitshift => return "Bitshift",
            Self::Prefix => return "Prefix",
        }
    }
}


/// Display for operator precedences
impl Display for OperatorPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


/// Token type for numeric literals
#[derive(Clone, PartialEq, Debug)]
pub enum NumericValue {
    Integer(u64),
    FloatingPoint(f64),
}

impl Display for NumericValue {
    fn fmt(&self, fout: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(fout, "Integer: '{}'", i),
            Self::FloatingPoint(f) => write!(fout, "Floating Point: '{}'", f),
        }
    }
}


/// Get the sring value of a token
pub trait ToStr {
    fn to_str(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Error)]
#[error("invalid string representation of keyword.")]
pub struct KeywordParseError;
impl FromStr for KeywordKind {
    type Err = KeywordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref STRING_KEYWORD_MAP: HashMap<&'static str, KeywordKind> = {
                let mut map = HashMap::new();

                for keyword in KeywordKind::iter() {
                    map.insert(keyword.as_str(), keyword);
                }

                map
            };
        };

        return STRING_KEYWORD_MAP.get(s).copied().ok_or(KeywordParseError);
    }
}

#[derive(Debug)]
pub struct PunctuatorLexerError;

impl FromStr for PunctuatorKind {
    type Err = PunctuatorLexerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref OPERATOR_STRING_MAP: HashMap<&'static str, PunctuatorKind> = {
                let mut map = HashMap::new();

                for puncuation in PunctuatorKind::iter() {
                    map.insert(puncuation.as_str(), puncuation);
                }

                map
            };
        };

        return OPERATOR_STRING_MAP.get(s).copied().ok_or(PunctuatorLexerError);
    }
}
