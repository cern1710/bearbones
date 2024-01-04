use crate::lexer::{Span};

#[derive(Debug)]
pub enum Error {
    EmptyChar(Span),
    CharNotAscii(Span),
    InvalidEscChar(Span),
    CharNotTerminated(Span),
    CharExpected(Span),
    UnexpectedEndOfInput
}