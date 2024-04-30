#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use viper_core::{source::SourceFile, token::{OperatorPrecedence, PunctuatorKind, KeywordKind, Numeric, Punctuator, StringLiteral, Token}};

    use crate::lexer::Lexer;

    #[test]
    fn lexer_simple() {
        let test_file = SourceFile::new_dummy(
            "define main(argc: i32, argv: String): i32 {}"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(
            Token::Keyword(KeywordKind::Define),
            Token::Identifier(String::from("main")),
            Token::Punctuator(
                PunctuatorKind::from_str("(").unwrap(), 
                None
            ),

            Token::Identifier(String::from("argc")),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Keyword(KeywordKind::I32),
            Token::Punctuator(
                PunctuatorKind::from_str(",").unwrap(), 
                None
            ),
            
            Token::Identifier(String::from("argv")),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Identifier(String::from("String")),
            
            Token::Punctuator(
                PunctuatorKind::from_str(")").unwrap(), 
                None
            ),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Keyword(KeywordKind::I32),
            Token::Punctuator(
                PunctuatorKind::from_str("{").unwrap(), 
                None
            ),
            Token::Punctuator(
                PunctuatorKind::from_str("}").unwrap(), 
                None
            ),
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

            Token::Keyword(KeywordKind::Let),
            Token::Identifier(String::from("str")),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Identifier(String::from("String")),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
            ),

            Token::StringLiteral(StringLiteral::from_str("\"test string literal\"").unwrap()),
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None
            ),
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
            "let str: String = \"test string literal\";\n
             let x: i32 = 5 * 2;"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(

            Token::Keyword(KeywordKind::Let),
            Token::Identifier(String::from("str")),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Identifier(String::from("String")),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
            ),

            Token::StringLiteral(StringLiteral::from_str("\"test string literal\"").unwrap()),
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None
            ),
            
            Token::Keyword(KeywordKind::Let),
            Token::Identifier(String::from("x")),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None
            ),
            Token::Keyword(KeywordKind::I32),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
            ),

            Token::Numeric{ i: Some(5), f: None},
            Token::Punctuator(
                PunctuatorKind::from_str("*").unwrap(), 
                Some(OperatorPrecedence::MulDivMod),
            ),
            Token::Numeric{ i: Some(2), f: None},
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None
            ),
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
