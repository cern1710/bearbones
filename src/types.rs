use crate::lexer::Spanned;

pub type Type = Spanned<TypeKind>;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    Bool,
    Int,
    Char,
    Array(Box<Type>),
}