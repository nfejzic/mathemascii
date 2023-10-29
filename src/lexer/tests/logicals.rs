use crate::lexer::keywords::logicals::Logical;

use super::super::*;

#[test]
fn and_or_not() {
    let src = "andornot and or not";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("and", TokenKind::Logical(Logical::And)),
            Token::new("or", TokenKind::Logical(Logical::Or)),
            Token::new("not", TokenKind::Logical(Logical::Not)),
            Token::new("and", TokenKind::Logical(Logical::And)),
            Token::new("or", TokenKind::Logical(Logical::Or)),
            Token::new("not", TokenKind::Logical(Logical::Not)),
        ]
    );
}

#[test]
fn implications() {
    let src = "implies <=> => iff";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("implies", TokenKind::Logical(Logical::Implies)),
            Token::new("<=>", TokenKind::Logical(Logical::IfAndOnlyIf)),
            Token::new("=>", TokenKind::Logical(Logical::Implies)),
            Token::new("iff", TokenKind::Logical(Logical::IfAndOnlyIf)),
        ]
    );
}

#[test]
fn quantors() {
    let src = "AA EE forallexists AAEE";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("AA", TokenKind::Logical(Logical::ForAll)),
            Token::new("EE", TokenKind::Logical(Logical::Exists)),
            Token::new("forall", TokenKind::Logical(Logical::ForAll)),
            Token::new("exists", TokenKind::Logical(Logical::Exists)),
            Token::new("AA", TokenKind::Logical(Logical::ForAll)),
            Token::new("EE", TokenKind::Logical(Logical::Exists)),
        ]
    );
}

#[test]
fn other() {
    let src = "_|_ bottop TT |-- |==";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("_|_", TokenKind::Logical(Logical::Bottom)),
            Token::new("bot", TokenKind::Logical(Logical::Bottom)),
            Token::new("top", TokenKind::Logical(Logical::Top)),
            Token::new("TT", TokenKind::Logical(Logical::Top)),
            Token::new("|--", TokenKind::Logical(Logical::VerticalDash)),
            Token::new("|==", TokenKind::Logical(Logical::Models)),
        ]
    );
}
