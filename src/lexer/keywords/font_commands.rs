use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    FontCommand,
    FontCommands,
    "bb" | "mathbf" => Bold,
    "bbb" | "mathbb" => BlackboardBold,
    "cc" | "mathcal" => Calligraphic,
    "tt" | "mathtt" => Typewriter,
    "fr" | "mathfrak" => Gothic,
    "sf" | "mathsf" => SansSerif,
    prefixes:
        Bold => "bbb"
);

impl From<FontCommand> for TokenKind {
    fn from(value: FontCommand) -> Self {
        TokenKind::FontCommand(value)
    }
}
