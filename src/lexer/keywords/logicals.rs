use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Logical,
    Logicals,
    "and" => And,
    "or" => Or,
    "not" | "neg" => Not,
    "=>" | "implies" => Implies,
    "if" => If,
    "<=>" | "iff" => IfAndOnlyIf,
    "AA" | "forall" => ForAll,
    "EE" | "exists" => Exists,
    "_|_" | "bot" => Bottom,
    "TT" | "top" => Top,
    "|--" | "vdash" => VerticalDash,
    "|==" | "models" => Models,
    prefixes:
        If => "iff"
);

impl From<Logical> for TokenKind {
    fn from(value: Logical) -> Self {
        TokenKind::Logical(value)
    }
}
