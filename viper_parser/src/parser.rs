use viper_core::token::Token;
use viper_lexer::lexer::Lexer;


#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn _new(lexer: &'a mut Lexer<'a>) -> Parser {
        let token = lexer.next_token();
        Parser {
            lexer,
            current_token: token,
        }
    }

    /// Advance the tokens 
    pub fn _eat(_token: &Token) {

    }
}
