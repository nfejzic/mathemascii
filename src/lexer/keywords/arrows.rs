use alemat::elements::Operator;

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Arrow,
    Arrows,
    "uarr" | "uparrow" => Up,
    "darr" | "downarrow" => Down,
    "->" | "to" | "rarr" | "rightarrow" => Right,
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
        Right => "->>"
);

impl From<Arrow> for TokenKind {
    fn from(value: Arrow) -> Self {
        TokenKind::Arrow(value)
    }
}

impl From<Arrow> for Operator {
    fn from(value: Arrow) -> Self {
        match value {
            Arrow::Up => Operator::from("↑"),
            Arrow::Down => Operator::from("↓"),
            Arrow::Right => Operator::rarrow(),
            Arrow::RightTail => Operator::from("↣"),
            Arrow::TwoHeadRight => Operator::from("↠"),
            Arrow::TwoHeadRightTail => Operator::from("⤖"),
            Arrow::MapsTo => Operator::from("↦"),
            Arrow::Left => Operator::larrow(),
            Arrow::LeftRight => Operator::from("↔"),
            Arrow::BigRight => Operator::from("⇒"),
            Arrow::BigLeft => Operator::from("⇐"),
            Arrow::BigLeftRight => Operator::from("⇔"),
        }
    }
}
