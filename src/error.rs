use crate::lexer::{Span};

// #[derive(Clone, Debug, PartialEq)]
pub enum Error {
    EmptyCharacter(Span),
}