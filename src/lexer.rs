use crate::error::Error;
use std::iter::Peekable;
use std::str::CharIndices;


#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Void,
    Char,
    Int,
    If,
    Else,
    For,
    Return,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,

    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Eqq,
    Neq,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Operator(Operator),
    Id(String),
    Char(char),
    Newline,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub const fn is_id(&self) -> bool {
        matches!(self.kind, TokenKind::Id(_))
    }

    pub fn id_name(&self) -> String {
        if let TokenKind::Id(name) = &self.kind {
            name.clone()
        } else {
            panic!("Token is not an identifier")
        }
    }
}

pub struct Lexer<'a> {
    cursor: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: input.char_indices().peekable(),
            tokens: Vec::with_capacity(input.len()),
            line: 1,
            col: 0,
            start: 0,
        }
    }

    fn col(&mut self, start: usize) -> usize {
        self.col = start - self.start;
        self.col
    }

    fn new_line(&mut self, ind: usize) {
        self.line += 1;
        self.start = ind + 1;
    }

    fn new_span(&mut self, start: usize, end: usize) -> Span {
        let end = start + end;
        let start_col = self.col(start);
        let end_col = self.col(end);

        let start = Pos {
            line: self.line,
            col: start_col,
        };

        let end = Pos {
            line: self.line,
            col: end_col,
        };

        Span { start, end }
    }

    fn consume_single_token(&mut self) {
        let (start, c) = self.cursor.next().unwrap();
        let kind = match c {
            '(' => TokenKind::Operator(Operator::LeftParen),
            ')' => TokenKind::Operator(Operator::RightParen),
            '{' => TokenKind::Operator(Operator::LeftBrace),
            '}' => TokenKind::Operator(Operator::RightBrace),
            ',' => TokenKind::Operator(Operator::Comma),
            ':' => TokenKind::Operator(Operator::Colon),
            ';' => TokenKind::Operator(Operator::Semicolon),
            '.' => TokenKind::Operator(Operator::Dot),
            _ => unreachable!(),
        };
        let span = self.new_span(start, c.len_utf8());
        self.tokens.push(Token::new(kind, span));
    }

    fn consume_next_char(&mut self, start: usize, len: usize,
            double: TokenKind, single: TokenKind, expected: char) -> Token {
        if let Some((_, ch)) = self.cursor.next_if(|x| x.1 == expected) {
            Token::new(double, self.new_span(start, ch.len_utf8()))
        } else {
            Token::new(single, self.new_span(start, len))
        }
    }

    fn check_eq_op(&mut self, start: usize, len: usize,
                    double: TokenKind, single: TokenKind) -> Token {
        self.consume_next_char(start, len, double, single, '=')
    }

    fn consume_double_token(&mut self) {
        let (start, c) = self.cursor.next().unwrap();
        let token = match c {
            '+' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::AddAssign),
                    TokenKind::Operator(Operator::Add)),
            '-' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::SubAssign),
                    TokenKind::Operator(Operator::Sub)),
            '*' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::MulAssign),
                    TokenKind::Operator(Operator::Mul)),
            '/' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::DivAssign),
                    TokenKind::Operator(Operator::Div)),
            '<' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::Le),
                    TokenKind::Operator(Operator::Lt)),
            '>' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::Ge),
                    TokenKind::Operator(Operator::Gt)),
            '!' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::Neq),
                    TokenKind::Operator(Operator::Not)),
            '=' => self.check_eq_op(start, c.len_utf8(),
                    TokenKind::Operator(Operator::Eqq),
                    TokenKind::Operator(Operator::Eq)),
            _ => unreachable!(),
        };
        self.tokens.push(token);
    }

    pub fn scanner(&mut self) -> Result<Vec<Token>, Error> {
        while let Some(&(start, c)) = self.cursor.peek() {
            match c {
                '(' | ')' | '{' | '}' | ',' | ':' | ';' | '.'
                => self.consume_single_token(),

                '+' | '-' | '*' | '/' | '<' | '>' | '!' | '='
                => self.consume_double_token(),

                ' ' | '\r' | '\t' => { self.cursor.next(); }

                '\n' => {
                    let (ind, _) = self.cursor.next().unwrap();
                    self.new_line(ind);
                }

                _ => {
                    self.cursor.next();
                    let span = self.new_span(start, c.len_utf8());
                    self.tokens.push(Token::new(TokenKind::Unknown, span));
                }
            }
        }
        Ok(self.tokens.clone())
    }
}