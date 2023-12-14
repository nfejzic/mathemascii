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

impl From<Logical> for alemat::elements::Operator {
    fn from(value: Logical) -> Self {
        use alemat::elements::Operator;

        match value {
            Logical::And => Operator::vee(),
            Logical::Or => Operator::wedge(),
            Logical::Not => Operator::not(),
            Logical::Implies => Operator::implies(),
            Logical::If => Operator::from("if"),
            Logical::IfAndOnlyIf => Operator::iff(),
            Logical::ForAll => Operator::forall(),
            Logical::Exists => Operator::exists(),
            Logical::Bottom => Operator::bottom(),
            Logical::Top => Operator::top(),
            Logical::VerticalDash => Operator::vdash(),
            Logical::Models => Operator::models(),
        }
    }
}
