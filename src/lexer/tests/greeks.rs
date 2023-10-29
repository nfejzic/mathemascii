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
