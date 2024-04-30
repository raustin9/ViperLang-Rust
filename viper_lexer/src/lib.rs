pub mod lexer;

#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use viper_core::{source::SourceFile, token::{Keyword, KeywordKind, Punctuator, StringLiteral, Token}};

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
                kind: KeywordKind::Define
            }),
            Token::Identifier { literal: String::from("main") },
            Token::Punctuator(Punctuator::from_str("(").unwrap()),

            Token::Identifier { literal: String::from("argc") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Keyword(Keyword { kind: KeywordKind::I32 }),
            Token::Punctuator(Punctuator::from_str(",").unwrap()),
            
            Token::Identifier { literal: String::from("argv") },
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Identifier { literal: String::from("String") },
            
            Token::Punctuator(Punctuator::from_str(")").unwrap()),
            Token::Punctuator(Punctuator::from_str(":").unwrap()),
            Token::Keyword(Keyword { kind: KeywordKind::I32 }),
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

            Token::Keyword(Keyword { kind: KeywordKind::Let }),
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
}
