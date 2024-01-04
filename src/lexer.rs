use crate::error::Error;
use phf::phf_map;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Const,
    Void,
    Bool,
    True,
    False,
    Char,
    Int,
    If,
    Else,
    For,
    While,
    Do,
    Continue,
    Break,
    Return,
}

const ESC_CHAR: [char; 7] = ['n', 'r', 't', '\\', '0', '\'', '\"'];

static KEYWORDS_MAP: phf::Map<&'static str, TokenKind> = phf_map! {
    "const" => TokenKind::Keyword(Keyword::Const),
    "void" => TokenKind::Keyword(Keyword::Void),
    "bool" => TokenKind::Keyword(Keyword::Bool),
    "true" => TokenKind::Bool(true),
    "false" => TokenKind::Bool(false),
    "char" => TokenKind::Keyword(Keyword::Char),
    "int" => TokenKind::Keyword(Keyword::Int),
    "if" => TokenKind::Keyword(Keyword::If),
    "else" => TokenKind::Keyword(Keyword::Else),
    "for" => TokenKind::Keyword(Keyword::For),
    "while" => TokenKind::Keyword(Keyword::While),
    "do" => TokenKind::Keyword(Keyword::Do),
    "continue" => TokenKind::Keyword(Keyword::Continue),
    "break" => TokenKind::Keyword(Keyword::Break),
    "return" => TokenKind::Keyword(Keyword::Return),
};

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
    Bool(bool),
    Int(i32),
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

    fn new_span(&mut self, start: usize, len: usize) -> Span {
        Span {
            start: Pos { line: self.line, col: self.col(start) },
            end: Pos { line: self.line, col: self.col(start + len) }
        }
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

    fn finish(&mut self, start: usize, c: char, len: usize)
                -> Result<Token, Error> {
        if self.cursor.next_if(|x| x.1 == '\'').is_some() {
            Ok(Token::new(TokenKind::Char(c),
                self.new_span(start, c.len_utf8())))
        } else {
            return Err(Error::CharNotTerminated(self.new_span(start, len)))
        }
    }

    fn check_valid(&mut self, start: usize, c: char) -> Result<(), Error> {
        if c == '\'' {
            Err(Error::EmptyChar(self.new_span(start, '\''.len_utf8())))
        } else if !c.is_ascii() {
            Err(Error::CharNotAscii(self.new_span(start, c.len_utf8())))
        } else {
            Ok(())
        }
    }

    fn scan_char(&mut self) -> Result<Token, Error> {
        let (start, c) = self.cursor.next().unwrap();
        self.check_valid(start, c)?;
        self.finish(start, c, c.len_utf8())
    }

    fn scan_esc(&mut self) -> Result<Token, Error> {
        let (start, _) =
            self.cursor.next().ok_or(Error::UnexpectedEndOfInput)?;
        let character = match self.cursor.next() {
            Some((_, c)) if ESC_CHAR.contains(&c) => match c {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => c,
            },
            Some((_, _))
                => return Err(Error::InvalidEscChar(self.new_span(start, 2))),
            None => return Err(Error::UnexpectedEndOfInput),
        };
        self.finish(start, character, 2)
    }

    fn consume_char(&mut self) -> Result<Token, Error> {
        let (start, _) =
            self.cursor.next().ok_or(Error::UnexpectedEndOfInput)?;
        if let Some((_ind, c)) = self.cursor.peek() {
            match *c {
                '\\' => self.scan_esc(),
                _ => self.scan_char(),
            }
        } else {
            Err(Error::CharExpected(self.new_span(start, 1)))
        }
    }

    fn consume_int(&mut self, start: usize) {
        let mut lexeme = String::new();
        while let Some((_, n)) = self.cursor.next_if(|x| x.1.is_ascii_digit()) {
            lexeme.push(n);
        }
        if self.cursor.peek().map_or(false, |x| x.1 == '.') {
            lexeme.push('.');
            let _ = self.cursor.next();
            while let Some((_, num)) =
                self.cursor.next_if(|x| x.1.is_ascii_digit()) {
                lexeme.push(num);
            }
        }
        let num = lexeme.parse::<i32>().expect("Unable to parse number.");
        let span = self.new_span(start, lexeme.len());
        self.tokens.push(Token::new(TokenKind::Int(num), span));
    }

    fn consume_id(&mut self, start: usize) {
        let mut lexeme = String::from("");
        while let Some((_, c)) = self
                    .cursor
                    .next_if(|x| x.1.is_ascii_alphanumeric() || x.1 == '_') {
            lexeme.push(c);
        }
        let len = lexeme.len();
        let span = self.new_span(start, len);
        let kind = KEYWORDS_MAP
                    .get(lexeme.as_str())
                    .cloned()
                    .unwrap_or(TokenKind::Id(lexeme));
        self.tokens.push(Token::new(kind, span));
    }

    fn consume_unknown(&mut self, start: usize, c: char) {
        self.cursor.next();
        let span = self.new_span(start, c.len_utf8());
        self.tokens.push(Token::new(TokenKind::Unknown, span));
    }

    pub fn scanner(&mut self) -> Result<Vec<Token>, Error> {
        while let Some(&(start, c)) = self.cursor.peek() {
            match c {
                '(' | ')' | '{' | '}' | ',' | ':' | ';' | '.'
                => self.consume_single_token(),
                '+' | '-' | '*' | '/' | '<' | '>' | '!' | '='
                => self.consume_double_token(),
                ' ' | '\r' | '\t' => {
                    self.cursor.next();
                }
                '\n' => {
                    let (ind, _) = self.cursor.next().unwrap();
                    self.new_line(ind);
                }
                '\'' => {
                    let token = self.consume_char()?;
                    self.tokens.push(token);
                }
                _ if c.is_ascii_digit() => self.consume_int(start),
                _ if c.is_ascii_alphanumeric() => self.consume_id(start),
                _ => self.consume_unknown(start, c),
            }
        }
        Ok(self.tokens.clone())
    }
}