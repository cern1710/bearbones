use bearbones::lexer::{Pos, Span, Lexer, TokenKind, Operator, Keyword};

#[cfg(test)]
mod test_lexer {
    use super::*;

    fn test_lexer(src: &str, expected_tokens: &[TokenKind]) -> bool {
        match Lexer::new(src).scanner() {
            Ok(tokens) => {
                let output = tokens
                    .iter()
                    .map(|t| t.kind.clone())
                    .collect::<Vec<TokenKind>>();
                println!("Output: {:#?}", output);
                output.as_slice() == expected_tokens
            }
            Err(error) => {
                eprintln!("{error:?}");
                false
            }
        }
    }

    #[test]
    fn single_char() {
        assert!(test_lexer(";", &[TokenKind::Operator(Operator::Semicolon)]))
    }

    #[test]
    fn character_literals() {
        assert!(test_lexer("'a'", &[TokenKind::Char('a')]))
    }

    #[test]
    fn true_false() {
        assert!(test_lexer("false", &[TokenKind::Bool(false)]))
    }

    #[test]
    fn function_decl1() {
        assert!(test_lexer(
            "
            int main() {
                return 0;
            }
            "
        , &[
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Id("main".into()),
            TokenKind::Operator(Operator::LeftParen),
            TokenKind::Operator(Operator::RightParen),
            TokenKind::Operator(Operator::LeftBrace),
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Int(0),
            TokenKind::Operator(Operator::Semicolon),
            TokenKind::Operator(Operator::RightBrace),
        ]))
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
        , &[
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
        ]))
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
}