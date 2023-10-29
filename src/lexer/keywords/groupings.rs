use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Grouping,
    Groupings,
    "(" => OpenParen,
    ")" => CloseParen,
    "[" => OpenBracket,
    "]" => CloseBracket,
    "{" => OpenBrace,
    "}" => CloseBrace,
    "(:" | "langle" | "<<" => LeftAngled,
    ":)" | "rangle" | ">>" => RightAngled,
    "{:" => OpenIgnored,
    ":}" => CloseIgnored,
    "abs" => Absolute,
    "floor" => Floor,
    "ceil" => Ceiling,
    "norm" => Norm,
    prefixes:
        OpenParen => "(:",
        OpenBrace => "{:"
);

impl From<Grouping> for TokenKind {
    fn from(value: Grouping) -> Self {
        TokenKind::Grouping(value)
    }
}
