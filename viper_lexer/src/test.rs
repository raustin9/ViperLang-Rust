#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use viper_core::{source::SourceFile, token::{Keyword, KeywordKind, Numeric, Punctuator, StringLiteral, Token}};

    use crate::lexer::Lexer;

    #[test]
    fn lexer_simple() {
        let test_file = SourceFile::new_dummy(
            "define main(argc: i32, argv: String): i32 {}"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(
            Token::Keyword(Keyword {
                kind: KeywordKind::Define , line: 1, column: 1 
            }),
            Token::Identifier { literal: String::from("main") },
            Token::Punctuator(Punctuator::from_str("(").unwrap()),

            Token::Identifier { literal: String::from("argc") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Keyword(Keyword { kind: KeywordKind::I32 , line: 1, column: 19 }),
            Token::Punctuator(Punctuator::from_str(",").unwrap()),
            
            Token::Identifier { literal: String::from("argv") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Identifier { literal: String::from("String") },
            
            Token::Punctuator(Punctuator::from_str(")").unwrap()),
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Keyword(Keyword { kind: KeywordKind::I32 , line: 1, column: 39  }),
            Token::Punctuator(Punctuator::from_str("{").unwrap()),
            Token::Punctuator(Punctuator::from_str("}").unwrap()),
        );

        let mut lexer = Lexer::new(&file_ptr);
        let mut token = lexer.next_token();
        let mut i = 0;
        while token != Token::EOF {
            println!("Token: {token}");
            assert_eq!(token, expected[i]);
            token = lexer.next_token();
            i += 1;
        }
    }
    
    #[test]
    fn lexer_string_literal() {
        let test_file = SourceFile::new_dummy(
            "let str: String = \"test string literal\";"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(

            Token::Keyword(Keyword { kind: KeywordKind::Let , line: 1, column: 1  }),
            Token::Identifier { literal: String::from("str") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Identifier { literal: String::from("String") },
            
            Token::Punctuator(Punctuator::from_str("=").unwrap()),

            Token::StringLiteral(StringLiteral::from_str("\"test string literal\"").unwrap()),
            Token::Punctuator(Punctuator::from_str(";").unwrap()),
        );

        let mut lexer = Lexer::new(&file_ptr);
        let mut token = lexer.next_token();
        let mut i = 0;
        while token != Token::EOF {
            println!("Token: {token}");
            assert_eq!(token, expected[i]);
            token = lexer.next_token();
            i += 1;
        }
    }
    
    #[test]
    fn lexer_numbers() {
        let test_file = SourceFile::new_dummy(
            "let str: String = \"test string literal\";\nlet x: i32 = 5 * 2;"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(

            Token::Keyword(Keyword { kind: KeywordKind::Let , line: 1, column: 1  }),
            Token::Identifier { literal: String::from("str") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Identifier { literal: String::from("String") },
            
            Token::Punctuator(Punctuator::from_str("=").unwrap()),

            Token::StringLiteral(StringLiteral::from_str("\"test string literal\"").unwrap()),
            Token::Punctuator(Punctuator::from_str(";").unwrap()),
            
            Token::Keyword(Keyword { kind: KeywordKind::Let , line: 2, column: 1  }),
            Token::Identifier { literal: String::from("x") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Keyword(Keyword { kind: KeywordKind::I32 , line: 2, column: 8  }),
            
            Token::Punctuator(Punctuator::from_str("=").unwrap()),

            Token::Numeric(Numeric::Integer { value: 5 }),
            Token::Punctuator(Punctuator::from_str("*").unwrap()),
            Token::Numeric(Numeric::Integer { value: 2 }),
            Token::Punctuator(Punctuator::from_str(";").unwrap()),
        );

        let mut lexer = Lexer::new(&file_ptr);
        let mut token = lexer.next_token();
        let mut i = 0;
        while token != Token::EOF {
            println!("Token: {token}");
            assert_eq!(token, expected[i]);
            token = lexer.next_token();
            i += 1;
        }
    }
}
