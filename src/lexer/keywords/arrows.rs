use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Arrow,
    Arrows,
    "uarr" | "uparrow" => Up,
    "darr" | "downarrow" => Down,
    "rarr" | "rightarrow" => Right,
    "->" | "to" => To,
    ">->" | "rightarrowtail" => RightTail,
    "->>" | "twoheadrightarrow" => TwoHeadRight,
    ">->>" | "twoheadrightarrowtail" => TwoHeadRightTail,
    "|->" | "mapsto" => MapsTo,
    "larr" | "leftarrow" => Left,
    "harr" | "leftrightarrow" => LeftRight,
    "rArr" | "Rightarrow" => BigRight,
    "lArr" | "Leftarrow" => BigLeft,
    "hArr" | "Leftrightarrow" => BigLeftRight,
    prefixes:
        RightTail => ">->>",
        TwoHeadRight => "twoheadrightarrowtail",
        To => "->>"
);

impl From<Arrow> for TokenKind {
    fn from(value: Arrow) -> Self {
        TokenKind::Arrow(value)
    }
}