use std::{iter::Peekable, str::{Chars, FromStr}, sync::Arc};
use substring::Substring;

use viper_core::{source::SourceFile, token::{Keyword, KeywordKind, Numeric, Punctuator, StringLiteral, Token}};

/// Lexer: This outputs a stream of Tokens from the input source code.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Pointer to the source code file
    source_file: &'a Arc<SourceFile>,

    /// Iterator that points to a location along the input source code
    code_iterator: Peekable<Chars<'a>>,
    
    /// The current line number that we are on in the file
    line_number: usize,

    /// The current column position within the line we are tokenizing
    column: usize,

    /// The current character that we are looking at when tokenizing
    current_char: char,

    /// The current number of characters that we have passed when tokenizing.
    /// This is used for creating substrings of parts of the source code
    position: usize,
}

/// Implementation of the Lexer for the 'a lifetime
impl<'a> Lexer<'a> {
    /// Create a new lexer from a pointer to the source code file
    pub fn new(source: &'a Arc<SourceFile>) -> Lexer {
        let mut it = source.code().chars().peekable();
        let c = it.next().unwrap();
        let l = Lexer {
            source_file: source,
            code_iterator: it,
            line_number: 1,
            column: 1,
            current_char: c,
            position: 0,
        };
    
        return l;
    }

    /// Read a string literal token from the source code input
    /// ex: "test string content"
    fn read_string_literal(&mut self) -> Token {
        let start_position = self.position.clone();
        self.read_char(); // eat the first "
        while self.current_char != '\"' {
            self.read_char();
        }

        self.read_char(); // eat the last "

        let s = self.source_file.code().substring(start_position, self.position);

        return Token::StringLiteral(StringLiteral::from_str(s).unwrap());
    }

    /// Read either a token for a specified keyword,
    /// and if it is not a valid keyword then it 
    /// is an identifier
    fn read_identifier(&mut self) -> Token {
        // println!("Lexer reading identifier");
        let start_position = self.position.clone();
        let start_column = self.column;

        while char::is_alphanumeric(self.current_char) {
            self.read_char();
        }

        let s = self.source_file.code().substring(start_position, self.position);
        match KeywordKind::from_str(s) {
            Ok(kind) => {
                // println!("Done.");
                return Token::Keyword(Keyword {
                    kind,
                    line: self.line_number,
                    column: start_column,
                });
            }
            Err(ref _err) => {
                // println!("Done.");
                return Token::Identifier { literal: String::from(s) };
            }
        }
    }

    /// Read a numeric value token and determine
    /// if it is a floating point or integer value
    pub fn read_number(&mut self) -> Token {
        let mut floating_point = false;
        let mut is_legal = true;
        let start_position = self.position.clone();

        while char::is_digit(self.current_char, 10) {
            if self.current_char == '.' && floating_point == true {
                is_legal = false;
            }

            if self.current_char == '.' {
                floating_point = true;
            }
            
            self.read_char();
        }

        if !is_legal {
            let s = self.source_file.code().substring(start_position, self.position);
            return Token::Illegal {
                line: self.line_number,
                column: self.column,
                content: s.into(),
            };
        } else if floating_point {
            let s = self.source_file.code().substring(start_position, self.position);
            return Token::Numeric(Numeric::FloatingPoint { 
                value: s.parse().unwrap()
            });
        }

        let s = self.source_file.code().substring(start_position, self.position);
        return Token::Numeric(Numeric::Integer { 
            value: s.parse().unwrap()
        });
    }

    /// Eat whitespace characters until we get to a non-whitespace 
    /// one in the source code input
    pub fn skip_whitespace(&mut self) {
        // println!("Done.");
        loop {
            match char::is_whitespace(self.current_char) {
                true => {
                    self.read_char();
                }

                false => return,
            }
        }
        /* Old implementation of this
        // println!("skipping whitespace");
        while  self.current_char == '\n'
                || self.current_char == '\r'
                || self.current_char == '\t'
                || self.current_char == ' ' 
        {
            if self.current_char == '\n' {
                self.line_number += 1;
            }
            // println!("skipping '{}'", self.current_char);
            self.read_char();
        }
        */
    }


    /// Eat a character and incriment proper values
    fn read_char(&mut self) {
        if self.code_iterator.peek().is_none() {
            self.current_char = '\0';
        } else {
            // println!("Read char current: '{}'", self.current_char);
            if self.current_char == '\n' {
                self.line_number += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.current_char = self.code_iterator.next().unwrap();
            
            self.position += 1;
        }
    }

    /// Peek ahead to the next character without 
    /// advanding our iterator
    fn peek_char(&mut self) -> char {
        match self.code_iterator.peek() {
            Some(c) => {
                return c.clone();
            }
            None => {
                return '\0';
            }
        }
    }

    /// Eat a single line comment
    fn _read_single_line_comment(&mut self) {
        while self.current_char != '\n' {
            self.read_char();
        }
    }

    /// Return a token from the source code
    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.current_char {
            '"' => {
                tok = self.read_string_literal();
            }

            '/' => {
                match self.peek_char() {
                    // TODO: Read comments
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("/=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("/").unwrap())
                    }
                }
            }
            
            '+' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("+=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("+").unwrap());
                    }
                }
            }

            '-' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("-=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("-").unwrap());
                    }
                }
            }
            
            '*' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("*=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("*").unwrap());
                    }
                }
            }
            
            '%' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("%=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("%").unwrap());
                    }
                }
            }

            '=' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("==").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("=").unwrap());
                    }
                }
            }

            '!' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("!=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("!").unwrap());
                    }
                }
            }

            '&' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("&=").unwrap());
                    }
                    '&' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("&&").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("&").unwrap());
                    }
                }
            }

            '|' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("|=").unwrap());
                    }
                    '|' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("||").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("|").unwrap());
                    }
                }
            }

            '^' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("^=").unwrap());
                    }
                    _ => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("^").unwrap());
                    }
                }
            }

            '~' => {
                match self.peek_char() {
                    '=' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("~=").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("~").unwrap());
                    }
                }
            }

            ':' => {
                match self.peek_char() {
                    ':' => {
                        self.read_char();
                        tok = Token::Punctuator(Punctuator::from_str("::").unwrap());
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str(":").unwrap());
                    }
                }
            }

            '(' => {
                tok = Token::Punctuator(Punctuator::from_str("(").unwrap());
            }

            ')' => {
                tok = Token::Punctuator(Punctuator::from_str(")").unwrap());
            }

            '[' => {
                tok = Token::Punctuator(Punctuator::from_str("[").unwrap());
            }

            ']' => {
                tok = Token::Punctuator(Punctuator::from_str("]").unwrap());
            }

            '{' => {
                tok = Token::Punctuator(Punctuator::from_str("{").unwrap());
            }

            '}' => {
                tok = Token::Punctuator(Punctuator::from_str("}").unwrap());
            }

            ',' => {
                tok = Token::Punctuator(Punctuator::from_str(",").unwrap());
            }

            '.' => {
                tok = Token::Punctuator(Punctuator::from_str(".").unwrap());
            }
            
            ';' => {
                tok = Token::Punctuator(Punctuator::from_str(";").unwrap());
            }

            '\0' => {
                tok = Token::EOF;
            }

            '<' => {
                match self.peek_char() {
                    '=' => {
                        tok = Token::Punctuator(Punctuator::from_str("<=").unwrap());
                    } 
                    '<' => {
                        self.read_char();
                        match self.peek_char() {
                            '=' => {
                                self.read_char();
                                tok = Token::Punctuator(Punctuator::from_str("<<=").unwrap());
                            }
                            _ => {
                                tok = Token::Punctuator(Punctuator::from_str("<=").unwrap());
                            }
                        }
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str("<").unwrap());
                    }
                }
            }

            '>' => {
                match self.peek_char() {
                    '=' => {
                        tok = Token::Punctuator(Punctuator::from_str(">=").unwrap());
                    } 
                    '>' => {
                        self.read_char();
                        match self.peek_char() {
                            '=' => {
                                self.read_char();
                                tok = Token::Punctuator(Punctuator::from_str(">>=").unwrap());
                            }
                            _ => {
                                tok = Token::Punctuator(Punctuator::from_str(">=").unwrap());
                            }
                        }
                    }
                    _ => {
                        tok = Token::Punctuator(Punctuator::from_str(">").unwrap());
                    }
                }
            }

            _ => {
                if char::is_digit(self.current_char, 10) {
                    tok = self.read_number();
                    return tok;
                } else if char::is_alphabetic(self.current_char) {
                    tok = self.read_identifier();
                    return tok;
                } else {
                    tok = Token::Illegal {
                        line: self.line_number,
                        column: self.column,
                        content: format!("Illegal character: '{}'", self.current_char).into()
                    };
                }
            }
        }
        self.read_char();

        return tok;
    }

    pub fn print_test(&self) {
        println!("LEXER FILE: {}", self.source_file);
    }
}
