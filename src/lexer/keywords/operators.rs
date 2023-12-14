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
    "//" => ForwardSlashLiteral,
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

impl From<Operator> for alemat::elements::Operator {
    fn from(value: Operator) -> Self {
        use alemat::elements::Operator;
        match value {
            self::Operator::Plus => Operator::plus(),
            self::Operator::Minus => Operator::minus(),
            self::Operator::Dot => Operator::dot(),
            self::Operator::Asterisk => Operator::asterisk(),
            self::Operator::Star => Operator::star(),
            self::Operator::ForwardSlashLiteral => Operator::from("/"),
            self::Operator::Backslash => Operator::from("\\"),
            self::Operator::Times => Operator::mult(),
            self::Operator::Divide => Operator::div(),
            self::Operator::LTimes => Operator::lfactor(),
            self::Operator::RTimes => Operator::rfactor(),
            self::Operator::Bowtie => Operator::bowtie(),
            self::Operator::Circle => Operator::ring(),
            self::Operator::OPlus => Operator::circle_plus(),
            self::Operator::OTimes => Operator::circle_times(),
            self::Operator::ODot => Operator::circle_dot(),
            self::Operator::Sum => Operator::sum(),
            self::Operator::Prod => Operator::prod(),
            self::Operator::Wedge => Operator::wedge(),
            self::Operator::BigWedge => Operator::big_wedge(),
            self::Operator::Cap => Operator::cap(),
            self::Operator::BigCap => Operator::big_cap(),
            self::Operator::Cup => Operator::cup(),
            self::Operator::BigCup => Operator::big_cup(),
        }
    }
}
