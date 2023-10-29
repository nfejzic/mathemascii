use crate::lexer::keywords::operators::Operator;

use super::super::*;

#[test]
fn plus_minus() {
    let src = "+ -";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("+", TokenKind::Operator(Operator::Plus)),
            Token::new("-", TokenKind::Operator(Operator::Minus)),
        ]
    );
}

#[test]
fn stars() {
    let src = "*** ** *";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("***", TokenKind::Operator(Operator::Star)),
            Token::new("**", TokenKind::Operator(Operator::Asterisk)),
            Token::new("*", TokenKind::Operator(Operator::Dot)),
        ]
    );
}

#[test]
fn ltimes_rtimes_bowtie() {
    let src = "|>< |><| ><|";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("|><", TokenKind::Operator(Operator::LTimes)),
            Token::new("|><|", TokenKind::Operator(Operator::Bowtie)),
            Token::new("><|", TokenKind::Operator(Operator::RTimes)),
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
    let src = "+ - @ox^^^^^";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("+", TokenKind::Operator(Operator::Plus)),
            Token::new("-", TokenKind::Operator(Operator::Minus)),
            Token::new("@", TokenKind::Operator(Operator::Circle)),
            Token::new("ox", TokenKind::Operator(Operator::OTimes)),
            Token::new("^^^", TokenKind::Operator(Operator::BigWedge)),
            Token::new("^^", TokenKind::Operator(Operator::Wedge)),
        ]
    );
}

#[test]
fn precedence() {
    let src = "nnnnn";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("nnn", TokenKind::Operator(Operator::BigCap)),
            Token::new("nn", TokenKind::Operator(Operator::Cap)),
        ]
    );
}
