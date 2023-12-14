use alemat::elements::Operator;

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Relation,
    Relations,
    "=" => Eq,
    "!=" | "ne" => NotEq,
    ":=" => Define,
    "<" | "lt" => LessThan,
    ">" | "gt" => GreaterThan,
    "<=" | "le" => LessEqualThan,
    ">=" | "ge" => GreaterEqualThan,
    "mlt" | "ll" => MuchLessThan,
    "mgt" | "gg" => MuchGreaterThan,
    "-<" | "prec" => Prec,
    "-<=" | "preceq" => PrecEq,
    ">-" | "succ" => Succ,
    ">-=" | "succeq" => SuccEq,
    "in" => In,
    "!in" | "notin" => NotIn,
    "sub" | "subset" => Subset,
    "sup" | "supset" => Superset,
    "sube" | "subseteq" => SubsetEq,
    "supe" | "supseteq" => SupersetEq,
    "_=" | "equiv" => Equivalent,
    "~=" | "cong" => Congruent,
    "~~" | "approx" => Approximate,
    "prop" | "propto" => Prop,
    prefixes:
        LessThan => "<=",
        GreaterThan => ">=", // or >-, or >-=
        Prec => "preceq", // or -<=, but preceq is longer that both -< and prec
        Succ => "succeq", // or >-=, but succeq is longer that both >- and succ
        Subset => "subseteq",
        Superset => "supseteq"
);

impl From<Relation> for TokenKind {
    fn from(value: Relation) -> Self {
        TokenKind::Relation(value)
    }
}

impl From<Relation> for Operator {
    fn from(value: Relation) -> Self {
        match value {
            Relation::Eq => Operator::eq(),
            Relation::NotEq => Operator::not_eq(),
            Relation::Define => Operator::from(":="),
            Relation::LessThan => Operator::lt(),
            Relation::GreaterThan => Operator::gt(),
            Relation::LessEqualThan => Operator::le(),
            Relation::GreaterEqualThan => Operator::ge(),
            Relation::MuchLessThan => Operator::from("m\u{003C}"),
            Relation::MuchGreaterThan => Operator::from("m\u{003E}"),
            Relation::Prec => Operator::prec(),
            Relation::PrecEq => Operator::preceq(),
            Relation::Succ => Operator::succ(),
            Relation::SuccEq => Operator::succeq(),
            Relation::In => Operator::in_set(),
            Relation::NotIn => Operator::not_in_set(),
            Relation::Subset => Operator::subset(),
            Relation::Superset => Operator::supset(),
            Relation::SubsetEq => Operator::subseteq(),
            Relation::SupersetEq => Operator::supseteq(),
            Relation::Equivalent => Operator::equivalent(),
            Relation::Congruent => Operator::congruent(),
            Relation::Approximate => Operator::approx(),
            Relation::Prop => Operator::propto(),
        }
    }
}
