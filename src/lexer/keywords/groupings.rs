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

impl Grouping {
    pub fn matches(&self, other: Self) -> bool {
        matches!(
            (*self, other),
            (Grouping::OpenParen, Grouping::CloseParen)
                | (Grouping::OpenParen, Grouping::CloseIgnored)
                | (Grouping::CloseParen, Grouping::OpenParen)
                | (Grouping::CloseParen, Grouping::OpenIgnored)
                | (Grouping::OpenBracket, Grouping::CloseBracket)
                | (Grouping::OpenBracket, Grouping::CloseIgnored)
                | (Grouping::CloseBracket, Grouping::OpenBracket)
                | (Grouping::CloseBracket, Grouping::OpenIgnored)
                | (Grouping::OpenBrace, Grouping::CloseBrace)
                | (Grouping::OpenBrace, Grouping::CloseIgnored)
                | (Grouping::CloseBrace, Grouping::OpenBrace)
                | (Grouping::CloseBrace, Grouping::OpenIgnored)
                | (Grouping::LeftAngled, Grouping::RightAngled)
                | (Grouping::LeftAngled, Grouping::CloseIgnored)
                | (Grouping::RightAngled, Grouping::LeftAngled)
                | (Grouping::RightAngled, Grouping::OpenIgnored)
                | (Grouping::OpenIgnored, Grouping::CloseParen)
                | (Grouping::OpenIgnored, Grouping::CloseBracket)
                | (Grouping::OpenIgnored, Grouping::CloseBrace)
                | (Grouping::OpenIgnored, Grouping::RightAngled)
                | (Grouping::OpenIgnored, Grouping::CloseIgnored)
                | (Grouping::CloseIgnored, Grouping::OpenParen)
                | (Grouping::CloseIgnored, Grouping::OpenBracket)
                | (Grouping::CloseIgnored, Grouping::OpenBrace)
                | (Grouping::CloseIgnored, Grouping::LeftAngled)
                | (Grouping::CloseIgnored, Grouping::OpenIgnored)
                | (Grouping::Absolute, Grouping::Absolute)
                | (Grouping::Floor, Grouping::Floor)
                | (Grouping::Ceiling, Grouping::Ceiling)
                | (Grouping::Norm, Grouping::Norm)
        )
    }
}
