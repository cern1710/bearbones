use crate::lexer::{Pos, Span, Token};

// #[derive(Clone, Debug, PartialEq)]
pub enum Error {
    EmptyCharacter(Span),
}