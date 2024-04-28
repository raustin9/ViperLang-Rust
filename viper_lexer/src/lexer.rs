use std::{str::Chars, sync::Arc};

use viper_core::{source::SourceFile, token::Token};

pub struct Lexer<'a> {
    source_file: &'a Arc<SourceFile>,
    current_position: usize,
    line_number: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Arc<SourceFile>) -> Lexer {
        Lexer {
            source_file: source,
            current_position: 0,
            line_number: 0,
            column: 0,
        }
    }

    pub fn skip_whitespace(&mut self) {
    }

    pub fn print_test(&self) {
        println!("LEXER FILE: {}", self.source_file);
    }
}
