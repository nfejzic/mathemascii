use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Accent,
    Accents,
    "hat" => Hat,
    "bar" | "overline" => Overline,
    "ul" | "underline" => Underline,
    "vec" => Vector,
    "tilde" => Tilde,
    "dot" => Dot,
    "ddot" => DoubleDot,
    "overset" => Overset,
    "underset" => Underset,
    "ubrace" | "underbrace" => Underbrace,
    "obrace" | "overbrace" => Overbrace,
    "color" => Color,
    "cancel" => Cancel
);

impl From<Accent> for TokenKind {
    fn from(value: Accent) -> Self {
        TokenKind::Accent(value)
    }
}
