pub mod test;

use std::{mem::swap, sync::Arc};

use viper_ast::{BinaryOperator, Expr, ExprNode, Stmt, StmtNode, UnaryOperator, VariableInitialization};
use viper_core::{error::ViperError, source::SourceFile, span::Span, token::{KeywordKind, NumericValue, OperatorPrecedence, PunctuatorKind, Token}};
use viper_lexer::lexer::Lexer;


/// A parser for parsing a file of the Viper programming language
#[derive(Debug)]
pub struct Parser<'a> {
    /// The [Lexer] that will hand the parser a token when asked
    lexer: Lexer<'a>,

    /// The source file that we are parsing
    source_file: &'a Arc<SourceFile>,

    /// The current token that we are looking at while parsing
    current_token: Token,

    /// Lookahead token for parsing
    peek_token: Token,
}

impl<'a> Parser<'a> {
    /// Create a new Parser from the input source file
    pub fn new(source: &'a Arc<SourceFile>) -> Parser {
        Parser {
            lexer: Lexer::new(source),
            source_file: source,
            current_token: Token::EOF,
            peek_token: Token::EOF,
        }
    }

    /// Parse a top-level (Program Scope) statement of a Viper source file
    pub fn parse_top_level(&mut self) -> Result<StmtNode, ViperError> {
        self.advance()?;
        match self.current_token {
            Token::Keyword(keyword, _) => {
                match keyword {
                    // Program-level variable initialization
                    KeywordKind::Let => {
                        println!("Parsing let statement");
                        return self.parse_variable_initialization();
                    }
                    _ => {
                        return Err(ViperError::ParserError);
                    }
                }
            }
            _ => {
                return Err(ViperError::ParserError);
            }
        }
    }

    fn parse_primary_expr(&mut self) -> Result<ExprNode, ViperError> {
        match self.current_token.clone() {
            Token::Punctuator(kind, _precedence, _span) => {
                match kind {
                    PunctuatorKind::Bang
                    | PunctuatorKind::Minus
                    | PunctuatorKind::Tilde => {
                        self.parse_expr_unary()
                    }
                    _ => Err(ViperError::ParserError)
                }
            }
            
            Token::Identifier(_name, _span) => {
                println!("Parsing Identifier!");
                self.parse_expr_identifier()
            }

            Token::NumericLiteral(value, _span) => {
                self.parse_number_literal(value)
//                match value {
//                    NumericValue::Integer(ivalue) => {
//                        self.advance()?;
//                        Ok(ExprNode::new(Expr::Integer(ivalue), span))
//                    }
//
//                    NumericValue::FloatingPoint(fvalue) => {
//                        Ok(ExprNode::new(Expr::Float(fvalue), span))
//                    }
//                }
            }
            
            _ => {
                Err(ViperError::ParserError)
            }
        }
    }
    
    /// Parse an expression for a unary operation
    fn parse_expr_unary(&mut self) -> Result<ExprNode, ViperError> {
        let operator = UnaryOperator::from(self.current_token.clone());
        self.advance()?; // eat the operator 
        
        let expr = self.parse_expr().unwrap();

        return Ok(ExprNode::new(Expr::UnaryOperation(operator, Arc::from(expr)), Span::dummy()));
    }

    /// Parse a variable declaration statement
    /// `let...`
    fn parse_variable_initialization(&mut self) -> Result<StmtNode, ViperError> {
        self.advance()?; // Eat the `let` token

        let ident_expr = self.parse_expr();
        self.advance()?; // eat the ':'

        let dtype = self.current_token.clone();
        self.advance()?;


        self.advance()?; // eat the '='
        let expr = self.parse_expr().unwrap();

        self.advance()?; // eat the ';'

        return Ok(
            StmtNode::new(
                Stmt::VariableInitialization(VariableInitialization::new(vec!(Arc::from(ident_expr.unwrap())), dtype, vec!(Arc::from(expr)))), 
                Span::dummy()
            )
        );
    }

    /// Parse an expression
    fn parse_expr(&mut self) -> Result<ExprNode, ViperError> {
        let mut lhs = self.parse_primary_expr()?;

        while get_operator_precedence(&self.current_token).is_some() {
            lhs = self.parse_expr_binary(&mut lhs, &get_operator_precedence(&self.current_token).unwrap())?;
        }

        return Ok(lhs);
    }

    fn parse_expr_binary(&mut self, lhs: &mut ExprNode, min_prec: &OperatorPrecedence) -> Result<ExprNode, ViperError> {
        let op = self.current_token.clone();
        self.advance()?;

        let mut rhs = self.parse_primary_expr()?;
        let next_prec = match get_operator_precedence(&self.current_token) {
            Some(p) => p,
            None => {
                return Ok(
                    ExprNode::new(
                        Expr::BinaryOperation(BinaryOperator::from(op.clone()), Arc::from(lhs.clone()), Arc::from(rhs)), 
                        Span::dummy()
                    )
                );
            }
        };

        if next_prec > *min_prec {
            rhs = self.parse_expr_binary(&mut rhs, &next_prec)?;
        }

        return Ok(
            ExprNode::new(
                Expr::BinaryOperation(BinaryOperator::from(op.clone()), Arc::from(lhs.clone()), Arc::from(rhs)), 
                Span::dummy()
            )
        );
    }

    fn parse_expr_identifier(&mut self) -> Result<ExprNode, ViperError> {
        match self.current_token.clone() {
            // Parse an identifier expression
            Token::Identifier(name, span) => {
                // TODO: Handle member field accesses
                self.advance()?;
                Ok(ExprNode::new(Expr::Identifier(name), span))
            }
            _ => {
                Err(ViperError::ParserError)
            }
        }
    }

    fn parse_number_literal(&mut self, value: NumericValue) -> Result<ExprNode, ViperError> {
        println!("Parsing number");
        self.advance()?;
        match value {
            NumericValue::Integer(value) => {
                Ok(ExprNode::new(Expr::Integer(value), Span::dummy()))
            }
            
            NumericValue::FloatingPoint(value) => {
                Ok(ExprNode::new(Expr::Float(value), Span::dummy()))
            }
        }
    }

    fn advance(&mut self) -> Result<(), ViperError> {
        println!("Eating: '{}'", self.current_token);
        self.current_token = self.lexer.next_token();
//        self.current_token = match self.peek_token {
//            Token::EOF => self.lexer.next_token(),
//            _ => {
//                let mut peek = Token::EOF;
//                swap(&mut peek, &mut self.peek_token);
//                peek
//            }
//        };

        Ok(())
    }
}

/// Get the [OperatorPrecedence] for the specified token
fn get_operator_precedence(op: &Token) -> Option<OperatorPrecedence> {
    match op {
        Token::Punctuator(_, prec, _) => prec.clone(),
        _ => None,
    }
}
