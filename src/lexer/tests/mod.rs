use crate::lexer::{
    keywords::greek::Greek,
    token::{Token, TokenKind},
    Tokenize,
};

mod arrows;
mod functions;
mod greeks;
mod logicals;
mod numbers;
mod operators;
mod relations;

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

#[test]
fn perf() {
    let src = "gammag gammag gammag gammag gammag ".repeat(1_000);
    let src = src.as_str();

    let tokens = src.tokenize();

    assert_eq!(tokens.count(), 10_000);
}
