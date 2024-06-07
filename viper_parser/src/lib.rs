pub mod test;

use std::{sync::Arc, rc::Rc, cell::RefCell};

use viper_ast::{BinaryOperator, Binding, CodeBlock, Conditional, Expr, ExprNode, FieldInit, ObjInit, ProcedureCall, ProcedureDef, StructDef, StructField, StructMethod, UnaryOperator, VariableInitialization, Visibility, WhileLoop};
use viper_core::{_type::Type, error::ViperError, scope::Scope, source::SourceFile, span::Span, symbol::Symbol, token::{KeywordKind, NumericValue, OperatorPrecedence, PunctuatorKind, Token}};
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
    _peek_token: Token,
}

impl<'a> Parser<'a> {
    /// Create a new Parser from the input source file
    pub fn new(source: &'a Arc<SourceFile>) -> Parser {
        Parser {
            lexer: Lexer::new(source),
            source_file: source,
            current_token: Token::EOF,
            _peek_token: Token::EOF,
        }
    }

    /// Parse a top-level (Program Scope) statement of a Viper source file
    pub fn parse_top_level(&mut self) -> Result<ExprNode, ViperError> {
        self.advance()?;
        match self.current_token {
            Token::Keyword(keyword, _) => {
                match keyword {
                    // Program-level variable initialization
                    KeywordKind::Let => {
                        println!("Parsing let statement");
                        let expr = self.parse_variable_initialization();
                        self.expect_punctuator(PunctuatorKind::SemiColon)?;
                        return expr;
                    }
                    KeywordKind::Define => {
                        println!("Parsing function definition");
                        return self.parse_procedure_definition();
                    }

                    KeywordKind::Struct => {
                        println!("Parsing struct definition");
                        return self.parse_struct_def();
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

    /// Parse a struct definition for the Viper programming language
    /// struct User {
    ///     name: string,
    ///     age: i32
    /// }
    fn parse_struct_def(&mut self) -> Result<ExprNode, ViperError> {
        let mut struct_vis = Visibility::Private;
        if &self.current_token == KeywordKind::Public {
            struct_vis = Visibility::Public;
            self.expect_keyword(KeywordKind::Public)?;
        }

        self.expect_keyword(KeywordKind::Struct)?;

        let ident = match self.current_token.clone() {
            Token::Identifier(name, _span) => {
                self.advance()?;
                name
            }
            _ => {
                return Err(ViperError::ParserError);
            }
        };

        let mut fields = vec![];
        let mut methods = vec![];

        self.expect_punctuator(PunctuatorKind::LSquirly)?;

        // Parse the fields and methods of the struct
        while &self.current_token != PunctuatorKind::RSquirly {
            // TODO: parse visibility
            let mut vis = Visibility::Private;
            if &self.current_token == KeywordKind::Public {
                vis = Visibility::Public;
                self.expect_keyword(KeywordKind::Public)?;
            }

            match &self.current_token {
                // If we are seeing a keyword, we should be 
                // parsing a method
                Token::Keyword(kind, _) => {
                    match kind {
                        KeywordKind::Method 
                        | KeywordKind::Static => {
                            methods.push(self.parse_struct_method(vis)?);
                            continue;
                        }
                        _ => return Err(ViperError::ParserError),
                    }
                }
               
                // If we are at an identifier, we should be
                // parsing a field
                Token::Identifier(_name, _span) => {
                    let binding = self.parse_binding()?;
                    
                    fields.push(
                        StructField::new(binding, vis)
                    );
                }

                _ => {
                    return Err(ViperError::ParserError);
                }
            }

            if &self.current_token != PunctuatorKind::Comma {
                if &self.current_token == PunctuatorKind::RSquirly {
                    break;
                }

                return Err(ViperError::ParserError);
            }

            self.expect_punctuator(PunctuatorKind::Comma)?;
        }

        self.expect_punctuator(PunctuatorKind::RSquirly)?;


        return Ok(ExprNode::new(
            Expr::StructDef(StructDef::new(
                    ident, 
                    Box::from(fields.as_slice()), 
                    Box::from(methods.as_slice()),
                    struct_vis,
                )), 
            Span::dummy()
        ));
    }

    /// Parse the methods within a struct in Viper
    fn parse_struct_method(&mut self, vis: Visibility) -> Result<StructMethod, ViperError> {
        let mut is_static = false;
        if &self.current_token == KeywordKind::Static {
            is_static = true;
            self.expect_keyword(KeywordKind::Static)?;
        } else {
            self.expect_keyword(KeywordKind::Method)?;
        }

        let mut params: Vec<Binding> = vec![];

        let ident = match &self.current_token {
            Token::Identifier(name, _span) => {
                name.clone()
            }
            _ => return Err(ViperError::ParserError),
        };

        self.advance()?; // eat the identifier

        self.expect_punctuator(PunctuatorKind::LParen)?;
        while &self.current_token != PunctuatorKind::RParen {
            params.push(self.parse_binding().unwrap());

            if &self.current_token != PunctuatorKind::Comma {
                if &self.current_token == PunctuatorKind::RParen {
                    break;
                } else {
                    return Err(ViperError::ParserError);
                }
            }

            self.expect_punctuator(PunctuatorKind::Comma)?;
        }
        self.expect_punctuator(PunctuatorKind::RParen)?;
        self.expect_punctuator(PunctuatorKind::Colon)?;

        let ret = self.parse_type()?;
        let body = self.parse_expr_block(Some(self.source_file.scope()))?;

        return Ok(StructMethod::new(
            ident, 
            Box::from(params.as_slice()), 
            Box::from(body), 
            ret, 
            vis, 
            is_static
        ));
    }

    /// Parse a type AST node
    fn parse_type(&mut self) -> Result<Type, ViperError> {
        let type_ast = self.current_token.clone();

        // Slice types
        if &self.current_token == PunctuatorKind::LBrace {
            return self.parse_type_slice();
        }

        // Pointer
        // *[type]
        if &self.current_token == PunctuatorKind::Star {
            self.expect_punctuator(PunctuatorKind::Star)?;
            return Ok(Type::Concrete { name: "Ref".to_string(), args: vec![self.parse_type()?] });
        }

        // TODO: parse the remainder of the types
        match &type_ast {
            Token::Keyword(kind, _span) => {
                self.advance()?;
                // TODO: Parse the arguments to the type
                return Ok(Type::Concrete { name: kind.as_str().to_string(), args: vec![] });
            }

            Token::Identifier(name, _span) => {
                self.advance()?;
                // TODO: parse the arguments to the type
                return Ok(Type::Concrete { name: name.clone(), args: vec![] });
            }
            _ => {
                return Err(ViperError::ParserError);
            }
        }
    }

    /// Parse the slice type in the Viper programming language
    fn parse_type_slice(&mut self) -> Result<Type, ViperError> {
        self.expect_punctuator(PunctuatorKind::LBrace)?;

        let mut args = vec![];

        while &self.current_token != PunctuatorKind::RBrace {
            let ty = self.parse_type()?;
            args.push(ty);

            if &self.current_token != PunctuatorKind::Comma {
                if &self.current_token == PunctuatorKind::RBrace {
                    break;
                } else {
                    return Err(ViperError::ParserError);
                }
            }
        }

        self.expect_punctuator(PunctuatorKind::RBrace)?;
        return Ok(Type::Concrete { name: "Slice".to_string(), args });
    }

    /// Parse expressions at their tighest bindings
    /// These 'primary' expressions get used to form larger ones
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
            }

            Token::StringLiteral(_literal, _span) => {
                self.parse_string_literal()
            }
            
            _ => {
                Err(ViperError::ParserError)
            }
        }
    }

    /// Parse a string liteal in Viper
    /// "string literal"
    fn parse_string_literal(&mut self) -> Result<ExprNode, ViperError> {
        match self.current_token.clone() {
            Token::StringLiteral(literal, span) => {
                self.advance()?;
                Ok(ExprNode::new(
                    Expr::StringLiteral(literal), span)
                )
            }

            _ => return Err(ViperError::ParserError),
        }
    }
    
    /// Parse an expression for a unary operation
    fn parse_expr_unary(&mut self) -> Result<ExprNode, ViperError> {
        let operator = UnaryOperator::from(self.current_token.clone());
        self.advance()?; // eat the operator 
        
        let expr = self.parse_expr().unwrap();

        return Ok(ExprNode::new(Expr::UnaryOperation(operator, Box::from(expr)), Span::dummy()));
    }

    /// Parse a variable declaration statement
    /// `let...`
    fn parse_variable_initialization(&mut self) -> Result<ExprNode, ViperError> {
        self.advance()?; // Eat the `let` token

        let ident_expr = self.parse_expr();
        self.advance()?; // eat the ':'

        let dtype = self.parse_type()?;
//        let dtype = self.current_token.clone();
//        self.advance()?;


        self.advance()?; // eat the '='
        let expr = self.parse_expr().unwrap();
        
        return Ok(
            ExprNode::new(
                Expr::Let(VariableInitialization::new(
                    vec!(Box::from(ident_expr.unwrap())),
                    dtype,
                    false,
                    vec!(Box::from(expr))
                )),
                Span::dummy()
            )
        );
    }

    /// Parse expressions that are meant to stand alone 
    fn parse_expr_stmt(&mut self, scope: Arc<RefCell<Scope>>) -> Result<ExprNode, ViperError> {
        match &self.current_token {
            Token::Keyword(kind, _span) => {
                match kind {
                    // Variable initialization
                    KeywordKind::Let => {
                        let expr = self.parse_variable_initialization();
                        self.expect_punctuator(PunctuatorKind::SemiColon)?;
                        return expr;
                    }
                    KeywordKind::While => {
                        let expr = self.parse_while_loop(scope);
                        return expr;
                    }
                    KeywordKind::Yield => {
                        let expr = self.parse_yield();
                        self.expect_punctuator(PunctuatorKind::SemiColon)?;
                        return expr;
                    }
                    KeywordKind::Return => {
                        let expr = self.parse_return();
                        self.expect_punctuator(PunctuatorKind::SemiColon)?;
                        return expr;
                    }
                    KeywordKind::If => {
                        let expr = self.parse_if(scope, KeywordKind::If);
                        return expr;
                    }
                    KeywordKind::Match => {
                        return self.parse_match();
                    }
                    KeywordKind::Defer => {
                        let expr = self.parse_defer();
                        self.expect_punctuator(PunctuatorKind::SemiColon)?;
                        return expr;
                    }
                    _ => {
                        return Err(ViperError::ParserError);
                    }
                }
            }

            _ => {
                let expr = self.parse_expr();
                self.expect_punctuator(PunctuatorKind::SemiColon)?;
                return expr;
            }
        }
    }

    /// Parse an if statement in Viper
    /// `
    /// if <condition> {...}
    ///
    /// if <condition> {
    /// } elif <condition> {
    /// } else {
    /// }
    fn parse_if(&mut self, parent: Arc<RefCell<Scope>>, expected: KeywordKind) -> Result<ExprNode, ViperError> {
        self.expect_keyword(expected)?;


        // We only want to parse a condition if we are an `if` or `elif` expr.
        // If we are an `else` expr, there is no condition to be evaluated
        let condition = if KeywordKind::Else != expected {
            Some(Rc::from(RefCell::new(self.parse_expr()?)))
        } else {
            None
        };
        
        let body = self.parse_expr_block(Some(parent.clone()))?;

        let else_clause = match self.current_token {
            Token::Keyword(ref kind, _) => {
                match kind {
                    KeywordKind::Elif => {
                        println!("ELIF");
                        Some(Rc::from(RefCell::new(self.parse_if(parent, KeywordKind::Elif)?)))
                    }
                    KeywordKind::Else => {
                        Some(Rc::from(RefCell::new(self.parse_if(parent, KeywordKind::Else)?)))
                    }
                    _ => {
                        None
                    }
                }
            }
            _ => {
                None
            }
        };

        return Ok(ExprNode::new(
            Expr::If(Conditional::new(
                condition,
                Rc::from(body),
                else_clause,
            )),
            Span::dummy()
        ));
    }

    /// Parse a conditional while loop for Viper
    /// `while [condition] {...}`
    /// `while 1 == 2-1 {...}`
    fn parse_while_loop(&mut self, parent: Arc<RefCell<Scope>>) -> Result<ExprNode, ViperError> {
        self.expect_keyword(KeywordKind::While)?;

        let condition = Box::from(self.parse_expr()?);
        let body = Box::from(self.parse_expr_block(Some(Arc::from(parent)))?);

        return Ok(ExprNode::new(Expr::WhileLoop(WhileLoop::new(condition, body)), Span::dummy()));
    }

    fn parse_match(&mut self) -> Result<ExprNode, ViperError> {
        todo!();
    }
    
    fn parse_defer(&mut self) -> Result<ExprNode, ViperError> {
        todo!();
    }
    
    fn _parse_switch(&mut self) -> Result<ExprNode, ViperError> {
        todo!();
    }
   
    /// Parse a return expression in Viper
    /// `return <expr>`
    /// `return 0`
    fn parse_return(&mut self) -> Result<ExprNode, ViperError> {
        self.expect_keyword(KeywordKind::Return)?;

        let expr = self.parse_expr()?;
        return Ok(ExprNode::new(Expr::Return(Box::from(expr)) ,Span::dummy()));
    }

    /// Parse a yield expression in Viper
    /// `yield true`
    /// `yield i + 1`
    fn parse_yield(&mut self) -> Result<ExprNode, ViperError> {
        self.expect_keyword(KeywordKind::Yield)?;
        let expr = self.parse_expr()?;
        
        return Ok(ExprNode::new(Expr::Yield(Box::from(expr)), Span::dummy()));
    }

    /// Parse a procedure definition
    /// This is for top-level procedures only not lambdas
    fn parse_procedure_definition(&mut self) -> Result<ExprNode, ViperError> {
        self.advance()?; // eat 'define'
        let mut params: Vec<Binding> = vec![];

        let ident = match self.current_token.clone() {
            Token::Identifier(name, _span) => {
                name
            }
            _ => {
                return Err(ViperError::ParserError);
            }
        };

        self.advance()?; // eat the identifier

        // Parse the parameters to the procedure
        self.advance()?; // eat the '('
        while &self.current_token != PunctuatorKind::RParen {
            params.push(self.parse_binding().unwrap());

            if &self.current_token != PunctuatorKind::Comma {
                if &self.current_token == PunctuatorKind::RParen {
                    break;
                } else {
                    return Err(ViperError::ParserError);
                }
            }

            self.expect_punctuator(PunctuatorKind::Comma)?;
        }
        self.expect_punctuator(PunctuatorKind::RParen)?;

        self.expect_punctuator(PunctuatorKind::Colon)?;

        let ret = self.parse_type()?;
//        let ret = self.current_token.clone();
//        self.advance()?;

        // Parse the function body 
        let body = self.parse_expr_block(Some(self.source_file.scope()))?;

        Ok(ExprNode::new(Expr::ProcedureDefinition(
            ProcedureDef::new(
                ident.clone(), 
                Box::from(params.as_slice()), 
                Box::from(body), 
                ret.clone(),
            )
        ), Span::dummy()))
    }

    /// Parse a code expression block
    /// {
    ///     let i: 32 = 10;
    ///     yield i + 15;
    /// }
    /// Expression blocks evaluate to values. This 
    /// is what the 'yield' keyword specifies.
    /// If no expression is yielded, then it yields
    /// the () unit type
    fn parse_expr_block(&mut self, parent: Option<Arc<RefCell<Scope>>>) -> Result<ExprNode, ViperError> {
        self.expect_punctuator(PunctuatorKind::LSquirly)?;
        let mut exprs = Vec::new();
        let scope = Arc::from(RefCell::new(Scope::new(parent)));
      
        // Read the expressions within the block
        while self.current_token != PunctuatorKind::RSquirly {
            match self.parse_expr_stmt(scope.clone()) {
                Ok(expr) => {
                    match expr.inner() {
                        Expr::Let(init) => {
                            let sym = init.to_symbol();
                            scope.borrow_mut().add_symbol(sym.get_key(), sym);
                            exprs.push(expr);
                        }

                        _ => {
                            exprs.push(expr);
                        }
                    }
                }

                Err(err) => {
                    return Err(err);
                }
            }
            // block.add_expr(self.parse_expr_stmt(block.scope()).unwrap());
        }
        self.expect_punctuator(PunctuatorKind::RSquirly)?;
        let block = CodeBlock::new(exprs, scope);


        Ok(ExprNode::new(Expr::CodeBlock(block), Span::dummy()))
    }

    /// Parse a binding in Viper
    /// a [Binding] is binding a type to an identifier
    /// `i: i32`
    /// `j: User`
    fn parse_binding(&mut self) -> Result<Binding, ViperError> {
        let ident = match self.current_token.clone() {
            Token::Identifier(name, _span) => {
                name
            }
            _ => {
                println!("Invalid token: '{}'. Expected identifier", &self.current_token);
                return Err(ViperError::ParserError);
            }
        };
        self.expect(&Token::Identifier("".into(), Span::dummy()))?;
        self.expect_punctuator(PunctuatorKind::Colon)?;

        let ty = &self.parse_type()?;
        
   
        Ok(Binding::new(ident.into(), ty.clone()))
    }

    /// Parse an expression
    fn parse_expr(&mut self) -> Result<ExprNode, ViperError> {
        let mut lhs = self.parse_primary_expr()?;

        while get_operator_precedence(&self.current_token).is_some() {
            lhs = self.parse_expr_binary(&mut lhs, &get_operator_precedence(&self.current_token).unwrap())?;
        }

        return Ok(lhs);
    }

    /// Parse an expression with an 'infix' operator
    /// a + b
    /// foo() - bar()
    fn parse_expr_binary(&mut self, lhs: &mut ExprNode, min_prec: &OperatorPrecedence) -> Result<ExprNode, ViperError> {
        let op = self.current_token.clone();
        self.advance()?;

        let mut rhs = self.parse_primary_expr()?;
        let next_prec = match get_operator_precedence(&self.current_token) {
            Some(p) => p,
            None => {
                return Ok(
                    ExprNode::new(
                        Expr::BinaryOperation(BinaryOperator::from(op.clone()), Box::from(lhs.clone()), Box::from(rhs)), 
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
                Expr::BinaryOperation(BinaryOperator::from(op.clone()), Box::from(lhs.clone()), Box::from(rhs)), 
                Span::dummy()
            )
        );
    }

    fn parse_expr_identifier(&mut self) -> Result<ExprNode, ViperError> {
        // Make sure that we are at an Identifier token
        match self.current_token.clone() {
            // We are... Parse an identifier expression
            Token::Identifier(ident, span) => {
                self.advance()?;
                let mut args: Vec<Box<ExprNode>> = Vec::new();

                // Switch on the types of tokens to see what type of 
                // expression we should be parsing
                match &self.current_token {
                    Token::Punctuator(kind, _prec, _span) => {
                        match kind {
                            // "identifier(..." is a function call
                            PunctuatorKind::LParen => {
                                self.expect_punctuator(PunctuatorKind::LParen)?;

                                // Parse the arguments
                                while &self.current_token != PunctuatorKind::RParen {
                                    match self.parse_expr() {
                                        Ok(expr) => {
                                            args.push(Box::from(expr));
                                        }
                                        Err (_err) =>{
                                            // If this errors assume that we did not parse an expression,
                                            // and that it is empty. 
                                            // This is not good and in the future we are going to have
                                            // a noop expression that will represent this
                                            continue;
                                        }
                                    }

                                    if &self.current_token != PunctuatorKind::Comma {
                                        if &self.current_token == PunctuatorKind::RParen {
                                            break;
                                        } else {
                                            // No comma, but no ')' is error
                                            return Err(ViperError::ParserError);
                                        }
                                    }

                                    self.expect_punctuator(PunctuatorKind::Comma)?;
                                }

                                self.expect_punctuator(PunctuatorKind::RParen)?;

                                // Return function call expression
                                return Ok(ExprNode::new(
                                        Expr::ProcedureCall(Box::from(ProcedureCall::new(ident, args))), 
                                        span
                                ));
                            }

                            // "identifier {..." is an object initialization
                            PunctuatorKind::LSquirly => {
                                self.expect_punctuator(PunctuatorKind::LSquirly)?;
                                let mut field_inits = vec![];
                                
                                while &self.current_token != PunctuatorKind::RSquirly {
                                    let field_name = match &self.current_token {
                                        Token::Identifier(name, _span) => {
                                            name.clone()
                                        }
                                        _ => return Err(ViperError::ParserError),
                                    };

                                    self.advance()?; // eat the identifier
                                    self.expect_punctuator(PunctuatorKind::Colon)?;
                                    
                                    let field_expr = self.parse_expr()?;
                                    
                                    field_inits.push(
                                        FieldInit::new(field_name, field_expr)
                                    );



                                    if &self.current_token != PunctuatorKind::Comma {
                                        if &self.current_token == PunctuatorKind::RSquirly {
                                            break;
                                        } else {
                                            // No comma, but no ')' is error
                                            return Err(ViperError::ParserError);
                                        }
                                    }

                                    self.expect_punctuator(PunctuatorKind::Comma)?;
                                }

                                self.expect_punctuator(PunctuatorKind::RSquirly)?;

                                return Ok(ExprNode::new(
                                    Expr::ObjInitialization(ObjInit::new(ident, field_inits))
                                    , span
                                ));
                            }
                            _ => {
                            }
                        }
                    }
                    _ => {
        
                    }
                }

                // See if we are at a function call
                if &self.current_token == PunctuatorKind::LParen {
                }
               
                // Return normal identifier expr
                Ok(ExprNode::new(Expr::Identifier(ident.clone()), span.clone()))
            }

            // We are not at an Identifier. Error
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

    fn expect_punctuator(&mut self, expected: PunctuatorKind) -> Result<(), ViperError> {
        if &self.current_token != expected {
            println!("Invalid Token {}. Expected {}", &self.current_token, expected);
            return Err(ViperError::ParserError);
        }

        println!("Eating: '{}'", self.current_token);
        self.current_token = self.lexer.next_token();

        Ok(())
    }

    fn expect_keyword(&mut self, expected: KeywordKind) -> Result<(), ViperError> {
        if self.current_token != expected {
            println!("Invalid Token {}. Expected {}", &self.current_token, expected);
            return Err(ViperError::ParserError);
        }

        println!("Eating: '{}'", self.current_token);
        self.current_token = self.lexer.next_token();

        Ok(())
    }

    /// Advance to the next token if we match the expected to the current token
    /// Otherwise we return an Error
    fn expect(&mut self, expected: &Token) -> Result<(), ViperError> {
        if expected != &self.current_token {
            println!("Invalid Token {}. Expected {}", &self.current_token, expected);
            return Err(ViperError::ParserError);
        }

        println!("Eating: '{}'", self.current_token);
        self.current_token = self.lexer.next_token();

        Ok(())
    }

    fn advance(&mut self) -> Result<(), ViperError> {
        println!("Eating: '{}'", self.current_token);
        self.current_token = self.lexer.next_token();

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
