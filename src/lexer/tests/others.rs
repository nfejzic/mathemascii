use crate::lexer::keywords::others::Other;

use super::super::*;

#[test]
fn roots() {
    let src = "sqrtroot";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("sqrt", TokenKind::Other(Other::SquareRoot)),
            Token::new("root", TokenKind::Other(Other::Root)),
        ]
    );
}

#[test]
fn calculus() {
    let src = "int oint del grad oo |...|";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("int", TokenKind::Other(Other::Integral)),
            Token::new("oint", TokenKind::Other(Other::OIntegral)),
            Token::new("del", TokenKind::Other(Other::Partial)),
            Token::new("grad", TokenKind::Other(Other::Nabla)),
            Token::new("oo", TokenKind::Other(Other::Infinity)),
            Token::new("|...|", TokenKind::Other(Other::LowDots)),
        ]
    );
}

#[test]
fn shapes() {
    let src = "square diamond triangle /_\\ frown /_";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("square", TokenKind::Other(Other::Square)),
            Token::new("diamond", TokenKind::Other(Other::Diamond)),
            Token::new("triangle", TokenKind::Other(Other::Triangle)),
            Token::new("/_\\", TokenKind::Other(Other::Triangle)),
            Token::new("frown", TokenKind::Other(Other::Frown)),
            Token::new("/_", TokenKind::Other(Other::Angle)),
        ]
    );
}

#[test]
fn number_sets() {
    let src = "CCNNQQRRZZ";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("CC", TokenKind::Other(Other::Complex)),
            Token::new("NN", TokenKind::Other(Other::Natural)),
            Token::new("QQ", TokenKind::Other(Other::Rational)),
            Token::new("RR", TokenKind::Other(Other::Irrational)),
            Token::new("ZZ", TokenKind::Other(Other::Whole)),
        ]
    );
}

#[test]
fn floor_ceil_power() {
    let src = "^|__ __| |~ rceiling +-";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("^", TokenKind::Other(Other::Power)),
            Token::new("|__", TokenKind::Other(Other::LeftFloor)),
            Token::new("__|", TokenKind::Other(Other::RightFloor)),
            Token::new("|~", TokenKind::Other(Other::LeftCeiling)),
            Token::new("rceiling", TokenKind::Other(Other::RightCeiling)),
            Token::new("+-", TokenKind::Other(Other::PlusMinus)),
        ]
    );
}
