use crate::lexer::keywords::relations::Relation;

use super::super::*;

#[test]
fn eq_neq() {
    let src = "= != =!=";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("=", TokenKind::Relation(Relation::Eq)),
            Token::new("!=", TokenKind::Relation(Relation::NotEq)),
            Token::new("=", TokenKind::Relation(Relation::Eq)),
            Token::new("!=", TokenKind::Relation(Relation::NotEq)),
        ]
    );
}

#[test]
fn compare() {
    let src = "< <= > >= mlt gg";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("<", TokenKind::Relation(Relation::LessThan)),
            Token::new("<=", TokenKind::Relation(Relation::LessEqualThan)),
            Token::new(">", TokenKind::Relation(Relation::GreaterThan)),
            Token::new(">=", TokenKind::Relation(Relation::GreaterEqualThan)),
            Token::new("mlt", TokenKind::Relation(Relation::MuchLessThan)),
            Token::new("gg", TokenKind::Relation(Relation::MuchGreaterThan)),
        ]
    );
}

#[test]
fn prec_suc() {
    let src = "-< -<= preceq prec succ >-=";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("-<", TokenKind::Relation(Relation::Prec)),
            Token::new("-<=", TokenKind::Relation(Relation::PrecEq)),
            Token::new("preceq", TokenKind::Relation(Relation::PrecEq)),
            Token::new("prec", TokenKind::Relation(Relation::Prec)),
            Token::new("succ", TokenKind::Relation(Relation::Succ)),
            Token::new(">-=", TokenKind::Relation(Relation::SuccEq)),
        ]
    );
}

#[test]
fn sets() {
    let src = "!in in sub sube supe";
    let tokens: Vec<_> = src.tokenize().collect();
    assert_eq!(
        tokens,
        vec![
            Token::new("!in", TokenKind::Relation(Relation::NotIn)),
            Token::new("in", TokenKind::Relation(Relation::In)),
            Token::new("sub", TokenKind::Relation(Relation::Subset)),
            Token::new("sube", TokenKind::Relation(Relation::SubsetEq)),
            Token::new("supe", TokenKind::Relation(Relation::SupersetEq)),
        ]
    );
}

#[test]
fn other() {
    let src = "_= ~= ~~ prop approx cong";

    let tokens: Vec<_> = src.tokenize().collect();

    assert_eq!(
        tokens,
        vec![
            Token::new("_=", TokenKind::Relation(Relation::Equivalent)),
            Token::new("~=", TokenKind::Relation(Relation::Congruent)),
            Token::new("~~", TokenKind::Relation(Relation::Approximate)),
            Token::new("prop", TokenKind::Relation(Relation::Prop)),
            Token::new("approx", TokenKind::Relation(Relation::Approximate)),
            Token::new("cong", TokenKind::Relation(Relation::Congruent)),
        ]
    );
}
