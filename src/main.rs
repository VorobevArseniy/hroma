use core::time;
use std::thread::sleep;

fn main() {
    let input = "let foo foo Foo = 42";
    let mut lexer = Lexer::new(input);

    lexer.get_tokens();
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    IntegerLiteral(i32),
    Ident(String),
    Type(String),
    BinaryOperator(String),
    LinLet(),
    Equal(),
    NonlinLet(),
    Colon(),
    LeftCurved(),
    RightCurved(),
    Arrow(),
    LeftParen(),
    RightParen(),
    Match(),
    Of(),
    Comma(),
    TypeKeyword(),
    Underlining(),
    EOI(),
    Error(),
}

enum Word {
    Ident,
    Type,
    Let,
    Match,
    Of,
    TypeKeyword,
    Underlining,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn inc(&mut self) {
        self.position += 1;
    }

    fn dec(&mut self) {
        self.position -= 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.inc();
            } else {
                break;
            }
        }
    }

    fn read_integer_literal(&mut self) -> i32 {
        let mut s = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                s.push(c);
                self.inc();
            } else {
                break;
            }
        }
        s.parse().unwrap()
    }

    fn read_word(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphabetic() || c == '_' || !s.is_empty() && c.is_ascii_digit() {
                s.push(c);
                self.inc();
            } else {
                break;
            }
        }
        s
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            Some(c) => match c {
                '+' => {
                    let mut s = String::new();
                    s.push(c);
                    Token::BinaryOperator(s)
                }
                '-' => {
                    self.inc();
                    if self.current_char().unwrap() == '>' {
                        self.dec();
                        Token::Arrow()
                    } else {
                        self.dec();
                        let mut s = String::new();
                        s.push(c);
                        Token::BinaryOperator(s)
                    }
                }
                '*' => {
                    let mut s = String::new();
                    s.push(c);
                    Token::BinaryOperator(s)
                }
                '/' => {
                    let mut s = String::new();
                    s.push(c);
                    Token::BinaryOperator(s)
                }
                '(' => Token::LeftParen(),
                ')' => Token::RightParen(),
                '{' => Token::LeftCurved(),
                '}' => Token::RightCurved(),
                '=' => Token::Equal(),
                ':' => Token::Colon(),
                ',' => Token::Comma(),
                _ => {
                    if c.is_ascii_digit() {
                        let num = self.read_integer_literal();
                        Token::IntegerLiteral(num)
                    } else if c.is_ascii_alphabetic() {
                        let s = self.read_word();
                        match type_word(&s) {
                            Word::Let => {
                                let c = self.current_char().unwrap();
                                if c == '!' {
                                    Token::NonlinLet()
                                } else {
                                    Token::LinLet()
                                }
                            }
                            Word::Ident => Token::Ident(s),
                            Word::Match => Token::Match(),
                            Word::Of => Token::Of(),
                            Word::Type => Token::Type(s),
                            Word::TypeKeyword => Token::TypeKeyword(),
                            Word::Underlining => Token::Underlining(),
                        }
                    } else {
                        panic!(
                            "Лексическая ошибка! Не удалось распознать символ на позиции {}",
                            self.position
                        );
                        // Token::Error()
                    }
                }
            },
            None => Token::EOI(),
        }
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        //let dur = time::Duration::from_secs(2);
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            println!("{:?}, {}", token, self.position);
            //sleep(dur);
            if token == Token::EOI() {
                break;
            } else {
                self.inc();
            }
        }
        tokens
    }
}

fn type_word(s: &String) -> Word {
    let tmp = s.as_str();
    match tmp {
        "let" => Word::Let,
        "match" => Word::Match,
        "of" => Word::Of,
        "type" => Word::TypeKeyword,
        "_" => Word::Underlining,
        _ => {
            if tmp.chars().nth(0).unwrap().is_ascii_lowercase() {
                Word::Ident
            } else {
                Word::Type
            }
        }
    }
}


