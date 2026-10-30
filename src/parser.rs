use crate::ast::*;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse the entire program
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    // Statements

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match &self.peek().token {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            Token::LeftBrace => self.parse_block(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Fn => self.parse_function(),
            Token::Return => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.consume(Token::Let, "Expected 'let'")?;

        let name = if let Token::Identifier(n) = &self.peek().token {
            let name = n.clone();
            self.advance();

            name
        } else {
            return Err(self.error("Expected identifier after 'let'"));
        };

        self.consume(Token::Equal, "Expected '=' after variable name")?;

        let value = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after value")?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.consume(Token::Print, "Expected 'print'")?;

        let expr = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Print(expr))
    }

    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after block")?;

        Ok(Stmt::Block(statements))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.consume(Token::If, "Expected 'if'")?;

        let condition = self.parse_expression()?;
        let then_branch = Box::new(self.parse_block()?);

        let else_branch = if self.matches(&Token::Else) {
            Some(Box::new(if self.check(&Token::If) {
                self.parse_if()?
            } else {
                self.parse_block()?
            }))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.consume(Token::While, "Expected 'while'")?;

        let condition = self.parse_expression()?;
        let body = Box::new(self.parse_block()?);

        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.consume(Token::While, "Expected 'for'")?;

        let variable = if let Token::Identifier(n) = &self.peek().token {
            let name = n.clone();
            self.advance();

            name
        } else {
            return Err(self.error("Expected identifier after 'for'"));
        };

        self.consume(Token::In, "Expected 'in' after loop variable")?;

        let iterable = self.parse_expression()?;
        let body = Box::new(self.parse_block()?);

        Ok(Stmt::For {
            variable,
            iterable,
            body,
        })
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.consume(Token::Fn, "Expected 'fn'")?;

        let name = if let Token::Identifier(n) = &self.peek().token {
            let name = n.clone();
            self.advance();

            name
        } else {
            return Err(self.error("Expected function name"));
        };

        self.consume(Token::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                if let Token::Identifier(p) = &self.peek().token {
                    params.push(p.clone());
                    self.advance();
                } else {
                    return Err(self.error("Expected parameter name"));
                }

                if !self.matches(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after function body")?;

        Ok(Stmt::Function { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.consume(Token::Return, "Expected 'return'")?;

        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Token::Semicolon, "Expected ';' after return value")?;

        Ok(Stmt::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression(expr))
    }

    // Expressions (with precedence)

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;

        if self.matches(&Token::Equal) {
            if let Expr::Identifier(name) = expr {
                let value = Box::new(self.parse_assignment()?);
                return Ok(Expr::Assign { name, value });
            } else {
                return Err(self.error("Invalid assignment target"));
            }
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;

        while self.matches(&Token::Or) {
            let right = Box::new(self.parse_and()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_equality()?;

        while self.matches(&Token::And) {
            let right = Box::new(self.parse_equality()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_tokens(&[Token::EqualEqual, Token::BangEqual]) {
            let binary_op = match op {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };

            let right = Box::new(self.parse_comparison()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: binary_op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.match_tokens(&[
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
        ]) {
            let binary_op = match op {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };

            let right = Box::new(self.parse_term()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: binary_op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_tokens(&[Token::Plus, Token::Minus]) {
            let binary_op = match op {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            let right = Box::new(self.parse_factor()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: binary_op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_tokens(&[Token::Star, Token::Slash, Token::Percent]) {
            let binary_op = match op {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };

            let right = Box::new(self.parse_unary()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                op: binary_op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(op) = self.match_tokens(&[Token::Minus, Token::Not, Token::Bang]) {
            let unary_op = match op {
                Token::Minus => UnaryOp::Minus,
                Token::Not | Token::Bang => UnaryOp::Not,
                _ => unreachable!(),
            };

            let expr = Box::new(self.parse_unary()?);

            return Ok(Expr::Unary { op: unary_op, expr });
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.matches(&Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.matches(&Token::LeftBracket) {
                let index = Box::new(self.parse_expression()?);
                self.consume(Token::RightBracket, "Expected ']' after index")?;

                expr = Expr::Index {
                    object: Box::new(expr),
                    index,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut args = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.matches(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after aarguments")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            args,
        })
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token {
            Token::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Nil => {
                self.advance();
                Ok(Expr::Nil)
            }
            Token::Number(n) => {
                let num = *n;
                self.advance();

                Ok(Expr::Number(num))
            }
            Token::String(s) => {
                let str = s.clone();
                self.advance();

                Ok(Expr::String(str.clone()))
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();

                Ok(Expr::Identifier(id))
            }
            Token::LeftParen => {
                self.advance();

                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after expresion")?;

                Ok(Expr::Grouping(Box::new(expr)))
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.matches(&Token::Comma) {
                            break;
                        }
                    }
                }

                self.consume(Token::RightBracket, "Expected ']' after list elements")?;

                Ok(Expr::List(elements))
            }
            Token::Fn => {
                self.advance();
                self.consume(Token::LeftParen, "Expected '(' after 'fn'")?;

                let mut params = Vec::new();

                if !self.check(&Token::RightParen) {
                    loop {
                        if let Token::Identifier(p) = &self.peek().token {
                            params.push(p.clone());
                            self.advance();
                        } else {
                            return Err(self.error("Expected parameter name"));
                        }

                        if !self.matches(&Token::Comma) {
                            break;
                        }
                    }
                }

                self.consume(Token::RightParen, "Expected ')' after parameters")?;
                self.consume(Token::LeftBrace, "Expected '{' before function body")?;

                let mut body = Vec::new();

                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.parse_statement()?);
                }

                self.consume(Token::RightBrace, "Expected '}' after function body")?;

                Ok(Expr::Lambda { params, body })
            }
            _ => Err(self.error("Expected expression")),
        }
    }

    // Helper methods

    // Check if token is Token::EOF
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token, Token::Eof)
    }

    // Peek the current token
    fn peek(&self) -> &TokenInfo {
        &self.tokens[self.current]
    }

    // Advance the token if it is not the Token::EOF
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<(), String> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token)
    }

    fn matches(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, tokens: &[Token]) -> Option<Token> {
        for token in tokens {
            if self.check(token) {
                let matched = self.peek().token.clone();
                self.advance();

                return Some(matched);
            }
        }

        None
    }

    // Error with current token details
    fn error(&self, message: &str) -> String {
        let token_info = self.peek();

        format!(
            "{} at line {}, column {}",
            message, token_info.line, token_info.column,
        )
    }
}
