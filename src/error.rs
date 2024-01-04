use crate::lexer::{Span};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    EmptyChar(Span),
    CharNotAscii(Span),
    EscNotFound(Span),
    CharNotTerminated(Span),
    CharExpected(Span),
    UnexpectedEndOfInput,
    MainNotFound,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, columns {} - {}",
            self.start.line, self.start.col, self.end.col
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            EmptyChar(span) => { writeln!(f, "Error: Character value is empty at {span}") }
            CharNotAscii(span) => { writeln!(f, "Token Error: Character not ASCII value at {span}") }
            EscNotFound(span) => { writeln!(f, "Syntax Error: Escape character not found at {span}") }
            CharNotTerminated(span) => { writeln!(f, "Syntax Error: Character literal not terminated at {span}") }
            CharExpected(span) => { writeln!(f, "Syntax Error: Character literal not found at {span}") }
            UnexpectedEndOfInput => { writeln!(f, "Token Error: ") }
            MainNotFound => { writeln!(f, "Error: 'main' function cannot be found") }
        }
    }
}