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
pub enum Symbol {
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
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Id(String),
    Char(char),
    Newline,
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

    fn scan_token(&mut self) {
        let (start, c) = self.cursor.next().unwrap();
        let kind = match c {
            '(' => TokenKind::Symbol(Symbol::LeftParen),
            _ => unreachable!(),
        };
        let span = self.new_span(start, c.len_utf8());
        self.tokens.push(Token::new(kind, span));
    }
}