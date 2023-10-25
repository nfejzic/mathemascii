use crate::lexer::{
    keywords::greek::Greek,
    token::{Token, TokenKind},
    Tokenize,
};

#[test]
fn skip_whitespace() {
    let src = "   alpha   24.42";
    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("alpha", TokenKind::Greek(Greek::Alpha)),
            Token::new("24.42", TokenKind::Number),
        ]
    );
}

mod number {
    use super::super::*;

    #[test]
    fn only_number() {
        let src = "24.42";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new("24.42", TokenKind::Number)));
    }

    #[test]
    fn start_with_dot() {
        let src = ".42";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new(".42", TokenKind::Number)));

        let src = ".42a";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new(".42", TokenKind::Number)));
    }

    #[test]
    fn start_and_stop_dot() {
        let src = ".42.3";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new(".42", TokenKind::Number)));
    }

    #[test]
    fn stop_at_non_digit() {
        let src = "24.42a";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new("24.42", TokenKind::Number)));
    }

    #[test]
    fn stop_at_dot() {
        let src = "24.42.";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), Some(Token::new("24.42", TokenKind::Number)));
    }
}

mod greek {
    use crate::lexer::keywords::greek::Greek;

    use super::super::*;

    #[test]
    fn greek_pi() {
        let src = "pi";
        let mut tokens = src.tokenize();
        assert_eq!(
            tokens.next(),
            Some(Token::new("pi", TokenKind::Greek(Greek::Pi)))
        );

        let src = "Pi";
        let mut tokens = src.tokenize();
        assert_eq!(
            tokens.next(),
            Some(Token::new("Pi", TokenKind::Greek(Greek::BigPi)))
        );
    }

    #[test]
    fn greek_alpha() {
        let src = "alpha";
        let mut tokens = src.tokenize();
        assert_eq!(
            tokens.next(),
            Some(Token::new("alpha", TokenKind::Greek(Greek::Alpha)))
        );
    }

    #[test]
    fn too_long() {
        let src = "abcdefghijklmnopqrstuvwxy";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn multiple() {
        let src = "alphabetagammadelta";

        let tokens: Vec<_> = src.tokenize().collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("alpha", TokenKind::Greek(Greek::Alpha)),
                Token::new("beta", TokenKind::Greek(Greek::Beta)),
                Token::new("gamma", TokenKind::Greek(Greek::Gamma)),
                Token::new("delta", TokenKind::Greek(Greek::Delta)),
            ]
        );
    }
}

mod arrow {
    use crate::lexer::keywords::arrows::Arrow;

    use super::super::*;

    #[test]
    fn right() {
        let src = "rarr rightarrow";
        let tokens: Vec<_> = src.tokenize().collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("rarr", TokenKind::Arrow(Arrow::Right)),
                Token::new("rightarrow", TokenKind::Arrow(Arrow::Right)),
            ]
        );
    }

    #[test]
    fn big_left() {
        let src = "lArrLeftarrow";
        let tokens: Vec<_> = src.tokenize().collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("lArr", TokenKind::Arrow(Arrow::BigLeft)),
                Token::new("Leftarrow", TokenKind::Arrow(Arrow::BigLeft)),
            ]
        );
    }

    #[test]
    fn too_long() {
        let src = "abcdefghijklmnopqrstuvwxy";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn multiple() {
        let src = "uarr->|->twoheadrightarrowtailto";

        let tokens: Vec<_> = src.tokenize().collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("uarr", TokenKind::Arrow(Arrow::Up)),
                Token::new("->", TokenKind::Arrow(Arrow::To)),
                Token::new("|->", TokenKind::Arrow(Arrow::MapsTo)),
                Token::new(
                    "twoheadrightarrowtail",
                    TokenKind::Arrow(Arrow::TwoHeadRightTail)
                ),
                Token::new("to", TokenKind::Arrow(Arrow::To)),
            ]
        );
    }
}

mod function {
    use crate::lexer::keywords::functions::Function;

    use super::super::*;

    #[test]
    fn sin_cos() {
        let src = "sincos";
        let tokens: Vec<_> = src.tokenize().collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("sin", TokenKind::Function(Function::Sin)),
                Token::new("cos", TokenKind::Function(Function::Cos)),
            ]
        );
    }

    #[test]
    fn f_g() {
        let src = "fg";
        let tokens: Vec<_> = src.tokenize().collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("f", TokenKind::Function(Function::F)),
                Token::new("g", TokenKind::Function(Function::G)),
            ]
        );
    }

    #[test]
    fn ln_log() {
        let src = "ln logln";
        let tokens: Vec<_> = src.tokenize().collect();
        assert_eq!(
            tokens,
            vec![
                Token::new("ln", TokenKind::Function(Function::Ln)),
                Token::new("log", TokenKind::Function(Function::Log)),
                Token::new("ln", TokenKind::Function(Function::Ln)),
            ]
        );
    }

    #[test]
    fn too_long() {
        let src = "abcdefghijklmnopqrstuvwxy";
        let mut tokens = src.tokenize();
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn multiple() {
        let src = "sincoshfg";

        let tokens: Vec<_> = src.tokenize().collect();

        assert_eq!(
            tokens,
            vec![
                Token::new("sin", TokenKind::Function(Function::Sin)),
                Token::new("cosh", TokenKind::Function(Function::CosH)),
                Token::new("f", TokenKind::Function(Function::F)),
                Token::new("g", TokenKind::Function(Function::G)),
            ]
        );
    }
}
