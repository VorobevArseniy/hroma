use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Литералы
    IntegerLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),

    // Идентификаторы и ключевые слова
    Ident(String),
    TypeIdent(String),
    KeywordLet,
    KeywordLetBang,
    KeywordType,
    KeywordMatch,
    KeywordOf,
    KeywordDefault,

    // Операторы и символы
    Equal,
    Colon,
    Arrow,
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,
    Comma,
    Underscore,

    // Служебные
    EOI,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOI;
        }

        let c = self.input[self.position];
        match c {
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' => self.read_word(),
            '=' => {
                self.advance();
                Token::Equal
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            '-' if self.peek() == '>' => {
                self.advance();
                self.advance();
                Token::Arrow
            }
            '{' => {
                self.advance();
                Token::LeftCurly
            }
            '}' => {
                self.advance();
                Token::RightCurly
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '_' => {
                self.advance();
                Token::Underscore
            }
            '!' => {
                self.advance();
                panic!("Unexpected character '!' at {}:{}", self.line, self.column)
            }
            '"' => self.read_string(),
            _ => panic!(
                "Unexpected character '{}' at {}:{}",
                c, self.line, self.column
            ),
        }
    }

    fn advance(&mut self) {
        if self.position < self.input.len() && self.input[self.position] == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }

    fn peek(&self) -> char {
        if self.position + 1 < self.input.len() {
            self.input[self.position + 1]
        } else {
            '\0'
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.advance();
        }

        if self.position < self.input.len() && self.input[self.position] == '.' {
            self.advance();
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.advance();
            }
            let s: String = self.input[start..self.position].iter().collect();
            Token::FloatLiteral(s.parse().unwrap())
        } else {
            let s: String = self.input[start..self.position].iter().collect();
            Token::IntegerLiteral(s.parse().unwrap())
        }
    }

    fn read_word(&mut self) -> Token {
        let start = self.position;

        // Читаем буквы, цифры и подчёркивания
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if !c.is_ascii_alphabetic() && c != '_' && !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        let word: String = self.input[start..self.position].iter().collect();

        // Специальная обработка `let!`
        if word == "let" && self.position < self.input.len() && self.input[self.position] == '!' {
            self.advance(); // Пропускаем '!'
            return Token::KeywordLetBang;
        }

        match word.as_str() {
            "let" => Token::KeywordLet,
            "type" => Token::KeywordType,
            "match" => Token::KeywordMatch,
            "of" => Token::KeywordOf,
            "default" => Token::KeywordDefault,
            _ if word.chars().next().unwrap().is_uppercase() => Token::TypeIdent(word),
            _ => Token::Ident(word),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Пропускаем открывающую кавычку
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.advance();
        }
        let s: String = self.input[start..self.position].iter().collect();
        if self.position < self.input.len() {
            self.advance(); // Пропускаем закрывающую кавычку
        }
        Token::StringLiteral(s)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_whitespace() {
                self.advance();
            } else if c == '#' {
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }
}
