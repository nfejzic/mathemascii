use crate::lexer::keywords::{functions::Function, greek::Greek};

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

#[test]
fn precedence() {
    let src = "gammag";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("gamma", TokenKind::Greek(Greek::Gamma)),
            Token::new("g", TokenKind::Function(Function::G)),
        ]
    );
}
