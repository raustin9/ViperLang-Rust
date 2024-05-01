#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use viper_core::{span::Span, source::SourceFile, token::{NumericValue, OperatorPrecedence, PunctuatorKind, KeywordKind, Token}};

    use crate::lexer::Lexer;

    #[test]
    fn lexer_simple() {
        let test_file = SourceFile::new_dummy(
            "define main(argc: i32, argv: String): i32 {}"
            , "Test File"
        );
        let file_ptr = Arc::from(test_file);

        let expected = vec!(
            Token::Keyword(KeywordKind::Define,Span::dummy()),
            Token::Identifier(String::from("main"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str("(").unwrap(), 
                None,
                Span::dummy()
            ),

            Token::Identifier(String::from("argc"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Keyword(KeywordKind::I32,Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(",").unwrap(), 
                None,
                Span::dummy()
            ),
            
            Token::Identifier(String::from("argv"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Identifier(String::from("String"),Span::dummy()),
            
            Token::Punctuator(
                PunctuatorKind::from_str(")").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Keyword(KeywordKind::I32,Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str("{").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Punctuator(
                PunctuatorKind::from_str("}").unwrap(), 
                None,
                Span::dummy()
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

            Token::Keyword(KeywordKind::Let,Span::dummy()),
            Token::Identifier(String::from("str"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Identifier(String::from("String"),Span::dummy()),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
                Span::dummy()
            ),

            Token::StringLiteral("\"test string literal\"".into(),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None,
                Span::dummy()
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

            Token::Keyword(KeywordKind::Let,Span::dummy()),
            Token::Identifier(String::from("str"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Identifier(String::from("String"),Span::dummy()),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
                Span::dummy()
            ),

            Token::StringLiteral("\"test string literal\"".into(),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None,
                Span::dummy()
            ),
            
            Token::Keyword(KeywordKind::Let,Span::dummy()),
            Token::Identifier(String::from("x"),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(":").unwrap(), 
                None,
                Span::dummy()
            ),
            Token::Keyword(KeywordKind::I32,Span::dummy()),
            
            Token::Punctuator(
                PunctuatorKind::from_str("=").unwrap(), 
                Some(OperatorPrecedence::Assign),
                Span::dummy()
            ),

            Token::NumericLiteral(NumericValue::Integer(5),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str("*").unwrap(), 
                Some(OperatorPrecedence::MulDivMod),
                Span::dummy()
            ),
            Token::NumericLiteral(NumericValue::Integer(2),Span::dummy()),
            Token::Punctuator(
                PunctuatorKind::from_str(";").unwrap(), 
                None,
                Span::dummy()
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
