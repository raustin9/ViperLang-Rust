use std::{iter::Peekable, str::{Chars, FromStr}, sync::Arc};
use substring::Substring;

use viper_core::{source::SourceFile, token::{Keyword, KeywordKind, Numeric, Punctuator, Token}};

pub struct Lexer<'a> {
    source_file: &'a Arc<SourceFile>,
    code_iterator: Peekable<Chars<'a>>,
    // code_iterator: std::str::PeekableChars<'a>,
    line_number: usize,
    column: usize,
    current_char: char,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Arc<SourceFile>) -> Lexer {
        // let it = source.code().chars().peekable();
        Lexer {
            source_file: source,
            code_iterator: source.code().chars().peekable(),
            line_number: 1,
            column: 1,
            current_char: source.code().chars().next().unwrap(),
            position: 0,
        }
    }

    /// Read either a token for a specified keyword,
    /// and if it is not a valid keyword then it 
    /// is an identifier
    pub fn read_identifier(&mut self) -> Token {
        let start_position = self.position;

        while char::is_alphabetic(self.current_char) {
            self.read_char();
        }

        let s = self.source_file.code().substring(start_position, self.position);
        match KeywordKind::from_str(s) {
            Ok(kind) => {
                return Token::Keyword(Keyword {
                    kind
                });
            }
            Err(ref _err) => {
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
        }

        if !is_legal {
            return Token::Illegal;
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
        loop {
            match char::is_whitespace(self.current_char) {
                true => {
                    if self.current_char == '\n' {
                        self.line_number += 1;
                        self.column = 1;
                    }

                    self.read_char();
                }

                false => return,
            }
        }
    }


    /// Eat a character and incriment proper values
    fn read_char(&mut self) {
        match self.code_iterator.next() {
            Some(c) => {
                self.current_char = c;
                self.column += 1;
                self.position += 1;
            }
            None => {
                self.current_char = '\0';
            }
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
    pub fn read_single_line_comment(&mut self) {
        while self.current_char != '\n' {
            self.read_char();
        }
    }

    /// Return a token from the source code
    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.current_char {
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
                } else if char::is_alphabetic(self.current_char) {
                    tok = self.read_identifier();
                } else {
                    tok = Token::Illegal;
                }
            }
        }

        return tok;
    }

    pub fn print_test(&self) {
        println!("LEXER FILE: {}", self.source_file);
    }
}
