use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Punctuator(Punctuator),
    Numeric(Numeric),
    StringLiteral(StringLiteral),
    Identifier {literal: String},
    Illegal,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(keyword) => {
                write!(f, "{}", keyword)
            }
            Self::Punctuator(punctuator) => {
                write!(f, "{}", punctuator)
            }
            Self::Numeric(numeric) => {
                write!(f, "{}", numeric)
            }
            Self::StringLiteral(string_literal) => {
                write!(f, "{}", string_literal)
            }
            Self::Identifier { literal } => {
                write!(f, "Identifier: '{}'", literal)
            }
            Self::Illegal => {
                write!(f, "Illegal")
            }
            Self::EOF => {
                write!(f, "EOF")

            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum KeywordKind {
    /// Declarator keywords
    Define,
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
    Assign,
    LogicalAndOr,
    Comparison,
    AddSub,
    MulDivMod,
    Bitshift,
    Prefix,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum PunctuatorKind {
    /// Operators
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
   
    /// Typical puncuation
    Comma,
    Colon,
    DoubleColon,
    Semicolon,
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
            Self::Colon => return ":",
            Self::DoubleColon => return "::",
            Self::Semicolon => return ";",
        }
    }
}

/// Represents a string literal
/// "string literal"
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct StringLiteral {
    literal: String,
}

/// Display the StringLiteral struct
impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String Literal: '{}'", self.literal)
    }
}

/// Token type for keywords in Viper
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Keyword {
    pub kind: KeywordKind,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keyword: '{}'", self.kind.as_str())
    }
}

/// Token type for punctuation in Viper
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Punctuator {
    pub value: String,
    pub kind: PunctuatorKind,
    pub precedence: Option<OperatorPrecedence>,
}

/// Display for operator precedences
impl Display for OperatorPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Implementation for operator precedences
impl OperatorPrecedence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lowest => return "Lowest",
            Self::Assign => return "Assign",
            Self::LogicalAndOr => return "LogicalAndOr",
            Self::Comparison => return "Comparison",
            Self::AddSub => return "AddSub",
            Self::MulDivMod => return "MulDivMod",
            Self::Bitshift => return "Bitshift",
            Self::Prefix => return "Prefix",
        }
    }
}

/// Display the Punctuator token
impl Display for Punctuator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.precedence {
            Some(ref prec) => {
                write!(f, "Punctuator: kind - {}. Precedence: {}", self.kind.as_str(), prec)
            }
            None => {
                write!(f, "Punctuator: kind - {}. Precedence: None", self.kind.as_str())
            }
        }
    }
}

/// Token type for numeric literals
#[derive(Clone, PartialEq)]
pub enum Numeric {
    Integer{value: u64},
    FloatingPoint{value: f64},
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer { value } => write!(f, "Integer: '{}'", value),
            Self::FloatingPoint { value } => write!(f, "Integer: '{}'", value),
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

impl FromStr for Punctuator {
    type Err = PunctuatorLexerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = PunctuatorKind::from_str(s).unwrap();
        let prec = match kind {
            PunctuatorKind::Plus | PunctuatorKind::Minus => {
                Some(OperatorPrecedence::AddSub)
            }
            PunctuatorKind::Mod | PunctuatorKind::Star | PunctuatorKind::Slash => {
                Some(OperatorPrecedence::MulDivMod)
            }
            PunctuatorKind::RShift | PunctuatorKind::LShift  | 
            PunctuatorKind::Caret | PunctuatorKind::Ampersand | 
            PunctuatorKind::Pipe => {
                Some(OperatorPrecedence::Bitshift)
            }
            PunctuatorKind::Bang | PunctuatorKind::Tilde => {
                Some(OperatorPrecedence::Prefix)
            }
            PunctuatorKind::LessThan | PunctuatorKind::GreaterThan |
            PunctuatorKind::LessThanEQ | PunctuatorKind::GreaterThanEQ |
            PunctuatorKind::EqualTo | PunctuatorKind::NotEqualTo => {
                Some(OperatorPrecedence::Comparison)
            }
            _ => {
                None
            }
        };

        return Ok(Punctuator {
            kind,
            precedence: prec,
            value: String::from(s)
        });
    }
}
