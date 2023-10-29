use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Relation,
    Relations,
    "=" => Eq,
    "!=" | "ne" => NotEq,
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
