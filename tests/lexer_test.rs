use bearbones::lexer::{Pos, Span, Lexer, TokenKind, Operator, Keyword};
use bearbones::error::Error;

#[cfg(test)]
mod test_lexer {
    use super::*;

    fn test_lexer(src: &str, expected: Result<Vec<TokenKind>, Error>) -> bool {
        let result = Lexer::new(src).scanner()
            .map(|tokens| tokens.iter().map(|t| t.kind.clone()).collect::<Vec<_>>());

        match (&result, &expected) {
            (Ok(output), Ok(expected_tokens)) => {
                if output == expected_tokens {
                    true
                } else {
                    println!("Test failed for input: {:?}", src);
                    println!("Expected tokens: {:#?}", expected_tokens);
                    println!("Received tokens: {:#?}", output);
                    false
                }
            },
            (Err(e), Err(expected_error)) => {
                if std::mem::discriminant(e) == std::mem::discriminant(expected_error) {
                    true
                } else {
                    println!("Test failed for input: {:?}", src);
                    println!("Expected error: {:#?}", expected_error);
                    println!("Received error: {:#?}", e);
                    false
                }
            },
            _ => {
                println!("Test failed for input: {:?}", src);
                println!("Expected: {:#?}", expected);
                println!("Received: {:#?}", result);
                false
            }
        }
    }



    #[test]
    fn single_char() {
        assert!(test_lexer(";", Ok(vec![TokenKind::Operator(Operator::Semicolon)])));
    }

    #[test]
    fn character_literals() {
        assert!(test_lexer("'a'", Ok(vec![TokenKind::Char('a')])));
    }

    #[test]
    fn true_false() {
        assert!(test_lexer("false", Ok(vec![TokenKind::Bool(false)])));
    }

    #[test]
    fn function_decl1() {
        assert!(test_lexer(
            "
            int main() {
                return 0;
            }
            "
        , Ok(vec![
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("main".into()),
            TokenKind::Operator(Operator::LeftParen),
            TokenKind::Operator(Operator::RightParen),
            TokenKind::Operator(Operator::LeftBrace),
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Int(0),
            TokenKind::Operator(Operator::Semicolon),
            TokenKind::Operator(Operator::RightBrace),
        ])))
    }

    #[test]
    fn function_decl2() {
        assert!(test_lexer(
            "
            int main(int a, int b) {
                int c;
                c = a + b;
                return 0;
            }
            "
        , Ok(vec![
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("main".into()),
            TokenKind::Operator(Operator::LeftParen),
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("a".into()),
            TokenKind::Operator(Operator::Comma),
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("b".into()),
            TokenKind::Operator(Operator::RightParen),
            TokenKind::Operator(Operator::LeftBrace),
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("c".into()),
            TokenKind::Operator(Operator::Semicolon),
            TokenKind::Id("c".into()),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Id("a".into()),
            TokenKind::Operator(Operator::Add),
            TokenKind::Id("b".into()),
            TokenKind::Operator(Operator::Semicolon),
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Int(0),
            TokenKind::Operator(Operator::Semicolon),
            TokenKind::Operator(Operator::RightBrace),
        ])))
    }

    #[test]
    fn test_token_spans() {
        let input = "const x = 5;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.scanner().unwrap();

        assert_eq!(tokens[0].span, Span { start: Pos { line: 1, col: 0 }, end: Pos { line: 1, col: 5 } });
        assert_eq!(tokens[1].span, Span { start: Pos { line: 1, col: 6 }, end: Pos { line: 1, col: 7 } });
        assert_eq!(tokens[2].span, Span { start: Pos { line: 1, col: 8 }, end: Pos { line: 1, col: 9 } });
        assert_eq!(tokens[3].span, Span { start: Pos { line: 1, col: 10 }, end: Pos { line: 1, col: 11 } });
        assert_eq!(tokens[4].span, Span { start: Pos { line: 1, col: 11 }, end: Pos { line: 1, col: 12 } });
    }

    #[test]
    fn empty_char() {
        assert!(test_lexer( "''", Err(Error::EmptyChar(Span {start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 2 }}))));
    }

    #[test]
    fn non_ascii() {
        assert!(test_lexer( "'ö'", Err(Error::CharNotAscii(Span {start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 3 }}))));
    }

    #[test]
    fn esc_not_found() {
        assert!(test_lexer( "'\\q'", Err(Error::EscNotFound(Span {start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 3 }}))));
    }

    #[test]
    fn char_not_terminated() {
        assert!(test_lexer( "'  '", Err(Error::CharNotTerminated(Span {start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 2 }}))));
    }

    #[test]
    fn char_expected() {
        assert!(test_lexer( "'", Err(Error::CharExpected(Span {start: Pos { line: 1, col: 1 }, end: Pos { line: 1, col: 2 }}))));
    }

    #[test]
    fn unexpected_end() {
        assert!(test_lexer( "'\\", Err(Error::UnexpectedEndOfInput)));
    }
}