use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Operator,
    Operators,
    "+" => Plus,
    "-" => Minus,
    "*" | "cdot" => Dot,
    "**" | "ast" => Asterisk,
    "***" | "star" => Star,
    "//" => ForwardSlash,
    "\\" | "backslash" | "setminus" => Backslash,
    "xx" | "times" => Times,
    "-:" | "div" => Divide,
    "|><" | "ltimes" => LTimes,
    "><|" | "rtimes" => RTimes,
    "|><|" | "bowtie" => Bowtie,
    "@" | "circ" => Circle,
    "o+" | "oplus" => OPlus,
    "ox" | "otimes" => OTimes,
    "o." | "odot" => ODot,
    "sum" => Sum,
    "prod" => Prod,
    "^^" | "wedge" => Wedge,
    "^^^" | "bigwedge" => BigWedge,
    "nn" | "cap" => Cap,
    "nnn" | "bigcap" => BigCap,
    "uu" | "cup" => Cup,
    "uuu" | "bigcup" => BigCup,
    prefixes:
        Dot => "**",
        Asterisk => "***",
        LTimes => "|><|",
        Wedge => "^^^",
        Cap => "nnn",
        Cup => "uuu"
);

impl From<Operator> for TokenKind {
    fn from(value: Operator) -> Self {
        TokenKind::Operator(value)
    }
}
