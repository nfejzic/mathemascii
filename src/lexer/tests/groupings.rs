use crate::lexer::keywords::groupings::Grouping;

use super::super::*;

#[test]
fn parens() {
    let src = "()(()))";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("(", TokenKind::Grouping(Grouping::OpenParen)),
            Token::new(")", TokenKind::Grouping(Grouping::CloseParen)),
            Token::new("(", TokenKind::Grouping(Grouping::OpenParen)),
            Token::new("(", TokenKind::Grouping(Grouping::OpenParen)),
            Token::new(")", TokenKind::Grouping(Grouping::CloseParen)),
            Token::new(")", TokenKind::Grouping(Grouping::CloseParen)),
            Token::new(")", TokenKind::Grouping(Grouping::CloseParen)),
        ]
    );
}

#[test]
fn brackets() {
    let src = "[][[]]][";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("[", TokenKind::Grouping(Grouping::OpenBracket)),
            Token::new("]", TokenKind::Grouping(Grouping::CloseBracket)),
            Token::new("[", TokenKind::Grouping(Grouping::OpenBracket)),
            Token::new("[", TokenKind::Grouping(Grouping::OpenBracket)),
            Token::new("]", TokenKind::Grouping(Grouping::CloseBracket)),
            Token::new("]", TokenKind::Grouping(Grouping::CloseBracket)),
            Token::new("]", TokenKind::Grouping(Grouping::CloseBracket)),
            Token::new("[", TokenKind::Grouping(Grouping::OpenBracket)),
        ]
    );
}

#[test]
fn angled() {
    let src = "langle(::)<<>>";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("langle", TokenKind::Grouping(Grouping::LeftAngled)),
            Token::new("(:", TokenKind::Grouping(Grouping::LeftAngled)),
            Token::new(":)", TokenKind::Grouping(Grouping::RightAngled)),
            Token::new("<<", TokenKind::Grouping(Grouping::LeftAngled)),
            Token::new(">>", TokenKind::Grouping(Grouping::RightAngled)),
        ]
    );
}

#[test]
fn ignored() {
    let src = "( :}{:)";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("(", TokenKind::Grouping(Grouping::OpenParen)),
            Token::new(":}", TokenKind::Grouping(Grouping::CloseIgnored)),
            Token::new("{:", TokenKind::Grouping(Grouping::OpenIgnored)),
            Token::new(")", TokenKind::Grouping(Grouping::CloseParen)),
        ]
    );
}
