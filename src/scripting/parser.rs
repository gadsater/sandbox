use super::ast::*;
use super::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.at_end() {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        Ok(Program { stmts })
    }

    // ----- helpers -----

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let tok = self.advance();
        if &tok == expected {
            Ok(())
        } else {
            Err(format!("expected {expected:?}, got {tok:?}"))
        }
    }

    fn at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.pos += 1;
        }
    }

    // ----- statements -----

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek().clone() {
            Token::Let => self.parse_let(),
            Token::Def => self.parse_def(),
            Token::Ret => self.parse_ret(),
            _ => {
                let expr = self.parse_expr()?;
                // Check for compound assignment (+=, -= etc.)
                match self.peek() {
                    Token::PlusEq => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Stmt::AssignOp {
                            target: expr,
                            op: BinOp::Add,
                            value: val,
                        })
                    }
                    Token::MinusEq => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Stmt::AssignOp {
                            target: expr,
                            op: BinOp::Sub,
                            value: val,
                        })
                    }
                    Token::StarEq => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Stmt::AssignOp {
                            target: expr,
                            op: BinOp::Mul,
                            value: val,
                        })
                    }
                    Token::SlashEq => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Stmt::AssignOp {
                            target: expr,
                            op: BinOp::Div,
                            value: val,
                        })
                    }
                    _ => Ok(Stmt::ExprStmt(expr)),
                }
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume `let`
        let name = match self.advance() {
            Token::Ident(n) => n,
            tok => return Err(format!("expected identifier after 'let', got {tok:?}")),
        };

        // Check for `on` keyword → method definition
        if matches!(self.peek(), Token::On) {
            self.advance(); // consume `on`
            let on_type = match self.advance() {
                Token::Ident(n) => n,
                tok => return Err(format!("expected type name after 'on', got {tok:?}")),
            };
            self.expect(&Token::ColonColon)?;
            self.expect(&Token::Fun)?;
            let (params, ret_ty) = self.parse_fun_signature()?;
            self.expect(&Token::Eq)?;
            self.skip_newlines();
            let body = self.parse_body()?;
            return Ok(Stmt::MethodDef {
                method_name: name,
                on_type,
                params,
                ret_ty,
                body,
            });
        }

        // Optional type annotation with ::
        let ty = if matches!(self.peek(), Token::ColonColon) {
            self.advance(); // consume ::
            // Check if it's a `fun` type → function definition
            if matches!(self.peek(), Token::Fun) {
                self.advance();
                let (params, ret_ty) = self.parse_fun_signature()?;
                self.expect(&Token::Eq)?;
                self.skip_newlines();
                let body = self.parse_body()?;
                return Ok(Stmt::Let {
                    name,
                    ty: Some(TypeAnnotation::Fun(
                        params.iter().map(|(_, t)| t.clone()).collect(),
                        ret_ty.map(|t| Box::new(t)),
                    )),
                    value: Expr::FunLit {
                        params,
                        ret_ty: None,
                        body,
                    },
                });
            }
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&Token::Eq)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Let { name, ty, value })
    }

    fn parse_def(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume `def`
        let name = match self.advance() {
            Token::Ident(n) => n,
            tok => return Err(format!("expected type name after 'def', got {tok:?}")),
        };
        self.expect(&Token::ColonColon)?;
        self.expect(&Token::Tup)?;
        self.expect(&Token::LParen)?;
        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RParen) {
            let fname = match self.advance() {
                Token::Ident(n) => n,
                tok => return Err(format!("expected field name, got {tok:?}")),
            };
            let ftype = self.parse_type()?;
            fields.push((fname, ftype));
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RParen)?;
        Ok(Stmt::DefTuple { name, fields })
    }

    fn parse_ret(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume `ret`
        let expr = self.parse_expr()?;
        Ok(Stmt::Return(expr))
    }

    /// Parse `(param type, ...)` and optional return type.
    fn parse_fun_signature(
        &mut self,
    ) -> Result<(Vec<(String, TypeAnnotation)>, Option<TypeAnnotation>), String> {
        self.expect(&Token::LParen)?;
        let mut params = Vec::new();
        while !matches!(self.peek(), Token::RParen) {
            let pname = match self.advance() {
                Token::Ident(n) => n,
                tok => return Err(format!("expected param name, got {tok:?}")),
            };
            let pty = self.parse_type()?;
            params.push((pname, pty));
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RParen)?;

        // Optional return type (identifier before `=`)
        let ret_ty = if matches!(self.peek(), Token::Ident(_)) {
            Some(self.parse_type()?)
        } else {
            None
        };
        Ok((params, ret_ty))
    }

    fn parse_type(&mut self) -> Result<TypeAnnotation, String> {
        match self.advance() {
            Token::Ident(name) => Ok(TypeAnnotation::Simple(name)),
            tok => Err(format!("expected type name, got {tok:?}")),
        }
    }

    /// Parse an indented body (statements until dedent / closing brace / blank line reduction).
    fn parse_body(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        // Simple approach: collect statements until we hit a blank-line break,
        // `Eof`, or a keyword that starts a new top-level declaration.
        loop {
            match self.peek() {
                Token::Eof | Token::RBrace => break,
                Token::Newline => {
                    self.advance();
                    // Two consecutive newlines end the body.
                    if matches!(self.peek(), Token::Newline | Token::Eof) {
                        break;
                    }
                    // If the next token starts a new top-level decl, stop.
                    if matches!(self.peek(), Token::Let | Token::Def) {
                        break;
                    }
                    continue;
                }
                _ => {}
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    // ----- expressions -----

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Token::And) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => BinOp::Eq,
                Token::BangEq => BinOp::Neq,
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::Le => BinOp::Le,
                Token::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Token::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Neg(Box::new(expr)));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    let field = match self.advance() {
                        Token::Ident(n) => n,
                        tok => return Err(format!("expected field name after '.', got {tok:?}")),
                    };
                    // Check for method call
                    if matches!(self.peek(), Token::LParen) {
                        self.advance();
                        let args = self.parse_arg_list()?;
                        expr = Expr::MethodCall {
                            object: Box::new(expr),
                            method: field,
                            args,
                        };
                    } else {
                        expr = Expr::FieldAccess {
                            object: Box::new(expr),
                            field,
                        };
                    }
                }
                Token::LParen => {
                    self.advance();
                    let args = self.parse_arg_list()?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::IntLit(n))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::FloatLit(f))
            }
            Token::Str(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::BoolLit(b))
            }
            Token::SelfKw => {
                self.advance();
                Ok(Expr::SelfRef)
            }
            Token::Fun => {
                self.advance();
                let (params, ret_ty) = self.parse_fun_signature()?;
                self.expect(&Token::Eq)?;
                self.skip_newlines();
                let body = self.parse_body()?;
                Ok(Expr::FunLit {
                    params,
                    ret_ty,
                    body,
                })
            }
            Token::Ident(name) => {
                self.advance();
                // Check for struct/tuple construction: `Name { ... }`
                if name.chars().next().map_or(false, |c| c.is_uppercase())
                    && matches!(self.peek(), Token::LBrace)
                {
                    self.advance(); // consume {
                    let fields = self.parse_field_init_list()?;
                    self.expect(&Token::RBrace)?;
                    return Ok(Expr::TupleConstruct {
                        type_name: name,
                        fields,
                    });
                }
                Ok(Expr::Ident(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("unexpected token in expression: {tok:?}")),
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        while !matches!(self.peek(), Token::RParen) {
            args.push(self.parse_expr()?);
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RParen)?;
        Ok(args)
    }

    fn parse_field_init_list(&mut self) -> Result<Vec<FieldInit>, String> {
        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            // Try named: `ident: expr`
            if let Token::Ident(name) = self.peek().clone() {
                // Look-ahead for `:`
                if self.tokens.get(self.pos + 1) == Some(&Token::Colon) {
                    self.advance(); // ident
                    self.advance(); // colon
                    let expr = self.parse_expr()?;
                    fields.push(FieldInit::Named(name, expr));
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                    continue;
                }
            }
            // Positional
            let expr = self.parse_expr()?;
            fields.push(FieldInit::Positional(expr));
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        Ok(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer::Lexer;
    use super::*;

    #[test]
    fn parse_let_int() {
        let tokens = Lexer::new("let x = 3").tokenize().unwrap();
        let prog = Parser::new(tokens).parse().unwrap();
        assert_eq!(prog.stmts.len(), 1);
    }

    #[test]
    fn parse_def_tuple() {
        let src = "def Point :: tup (x int, y int, z int)";
        let tokens = Lexer::new(src).tokenize().unwrap();
        let prog = Parser::new(tokens).parse().unwrap();
        assert!(matches!(prog.stmts[0], Stmt::DefTuple { .. }));
    }

    #[test]
    fn parse_function_def() {
        let src = "let factorial :: fun (x int) int =\n    ret x * factorial(x - 1)";
        let tokens = Lexer::new(src).tokenize().unwrap();
        let prog = Parser::new(tokens).parse().unwrap();
        assert!(matches!(prog.stmts[0], Stmt::Let { .. }));
    }
}
