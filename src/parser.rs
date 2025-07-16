use super::lexer::{Lexer, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Function {
    LinFunc {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Function>,
        return_expr: Vec<Expr>,
    },
    NonlinFunc {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Function>,
        return_expr: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    Call(String, Vec<Expr>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = Some(lexer.next_token());
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse(&mut self) -> Vec<Function> {
        let mut functions = Vec::new();
        while self.current_token != Token::EOI {
            functions.push(self.parse_function());
        }
        functions
    }

    fn parse_function(&mut self) -> Function {
        let is_nonlin = matches!(self.current_token, Token::KeywordLetBang);
        self.advance();

        let name = self.parse_ident();
        self.expect(Token::Equal);

        let (params, body, return_expr) = self.parse_function_body();

        if is_nonlin {
            Function::NonlinFunc {
                name,
                params,
                body,
                return_expr,
            }
        } else {
            Function::LinFunc {
                name,
                params,
                body,
                return_expr,
            }
        }
    }

    fn parse_function_body(&mut self) -> (Vec<(String, Type)>, Vec<Function>, Vec<Expr>) {
        // Проверяем синтаксис лямбды: ident: type -> ...
        if let Token::Ident(_) = &self.current_token {
            if let Some(Token::Colon) = &self.peek_token {
                return self.parse_lambda_style();
            }
        }

        match &self.current_token {
            Token::IntegerLiteral(_) => {
                let n = if let Token::IntegerLiteral(n) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    n
                } else {
                    unreachable!()
                };
                self.advance();
                (vec![], vec![], vec![Expr::IntLiteral(n)])
            }
            Token::FloatLiteral(_) => {
                let f = if let Token::FloatLiteral(f) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    f
                } else {
                    unreachable!()
                };
                self.advance();
                (vec![], vec![], vec![Expr::FloatLiteral(f)])
            }
            Token::StringLiteral(_) => {
                let s = if let Token::StringLiteral(s) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    s
                } else {
                    unreachable!()
                };
                self.advance();
                (vec![], vec![], vec![Expr::StringLiteral(s)])
            }
            Token::LeftCurly => {
                self.expect(Token::LeftCurly);
                let params = Vec::new();
                let mut body = Vec::new();
                let mut return_expr = Vec::new();

                while !matches!(&self.current_token, Token::RightCurly | Token::EOI) {
                    if matches!(
                        &self.current_token,
                        Token::KeywordLet | Token::KeywordLetBang
                    ) {
                        body.push(self.parse_function());
                    } else {
                        return_expr.push(self.parse_expr());
                    }
                }

                self.expect(Token::RightCurly);
                (params, body, return_expr)
            }
            _ => {
                let expr = self.parse_expr();
                (vec![], vec![], vec![expr])
            }
        }
    }

    fn parse_lambda_style(&mut self) -> (Vec<(String, Type)>, Vec<Function>, Vec<Expr>) {
        let mut params = Vec::new();

        // Парсим параметры в формате a: Int, b: Float
        loop {
            let name = self.parse_ident();
            self.expect(Token::Colon);
            let typ = self.parse_type();
            params.push((name, typ));

            match self.current_token {
                Token::Comma => {
                    self.advance();
                    continue;
                }
                Token::Arrow => break,
                _ => panic!("Expected ',' or '->' after parameter"),
            }
        }

        self.expect(Token::Arrow);
        let return_expr = vec![self.parse_expr()];

        (params, vec![], return_expr)
    }

    fn parse_type(&mut self) -> Type {
        match &self.current_token {
            Token::TypeIdent(name) if name == "Int" => {
                self.advance();
                Type::Int
            }
            Token::TypeIdent(name) if name == "Float" => {
                self.advance();
                Type::Float
            }
            Token::TypeIdent(name) if name == "String" => {
                self.advance();
                Type::String
            }
            _ => panic!("Expected type (Int, Float or String)"),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match &self.current_token {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                if self.current_token == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();

                    while self.current_token != Token::RightParen {
                        args.push(self.parse_expr());
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    }

                    self.expect(Token::RightParen);
                    Expr::Call(name, args)
                } else {
                    Expr::Call(name, vec![])
                }
            }
            Token::IntegerLiteral(_) => {
                let n = if let Token::IntegerLiteral(n) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    n
                } else {
                    unreachable!()
                };
                self.advance();
                Expr::IntLiteral(n)
            }
            Token::FloatLiteral(_) => {
                let f = if let Token::FloatLiteral(f) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    f
                } else {
                    unreachable!()
                };
                self.advance();
                Expr::FloatLiteral(f)
            }
            Token::StringLiteral(_) => {
                let s = if let Token::StringLiteral(s) =
                    std::mem::replace(&mut self.current_token, Token::EOI)
                {
                    s
                } else {
                    unreachable!()
                };
                self.advance();
                Expr::StringLiteral(s)
            }
            _ => panic!("Unexpected token in expression: {:?}", self.current_token),
        }
    }

    fn advance(&mut self) {
        self.current_token = self.peek_token.take().unwrap_or(Token::EOI);
        self.peek_token = Some(self.lexer.next_token());
    }

    fn expect(&mut self, expected: Token) {
        if self.current_token != expected {
            panic!("Expected {:?}, got {:?}", expected, self.current_token);
        }
        self.advance();
    }

    fn parse_ident(&mut self) -> String {
        if let Token::Ident(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected identifier");
        }
    }
}
