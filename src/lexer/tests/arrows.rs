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
