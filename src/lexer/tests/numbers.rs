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
