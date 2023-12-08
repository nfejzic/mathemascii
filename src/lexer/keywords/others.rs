//! This corresponds to the `Miscellaneous symbols` table at [Ascii math syntax
//! page](http://asciimath.org/#syntax).

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Other,
    Others,
    "," => Comma,
    "'" => Prime,
    "frac" => Fraction,
    "/" => ForwardSlash,
    "^" => Power,
    "_" => Subscript,
    "sqrt" => SquareRoot,
    "root" => Root,
    "int" => Integral,
    "oint" => OIntegral,
    "del" | "partial" => Partial,
    "grad" | "nabla" => Nabla,
    "+-" | "pm" => PlusMinus,
    "O/" | "emptyset" => EmptySet,
    "oo" | "infty" => Infinity,
    "aleph" => Aleph,
    ":." | "therefore" => Therefore,
    ":'" | "because" => Because,
    "|...|" | "|ldots|" => LowDots,
    "|cdots|" => CenterDots,
    "vdots" => VerticalDots,
    "ddots" => DiagonalDots,
    "|" => VerticalBar,
    "|\\|" => VerticalBars,
    "|quad|" => VerticalBarsWide,
    "/_" => Angle,
    "frown" => Frown,
    "/_\\" | "triangle" => Triangle,
    "diamond" => Diamond,
    "square" => Square,
    "|__" | "lfloor" => LeftFloor,
    "__|" | "rfloor" => RightFloor,
    "|~" | "lceiling" => LeftCeiling,
    "~|" | "rceiling" => RightCeiling,
    "CC" => Complex,
    "NN" => Natural,
    "QQ" => Rational,
    "RR" => Irrational,
    "ZZ" => Integer,
    "text" => Text,
    "\"" => Quote,
    prefixes:
        VerticalBar => "|\\|",
        ForwardSlash => "/_",
        Subscript => "__|",
        Angle => "/_\\"
);

impl From<Other> for TokenKind {
    fn from(value: Other) -> Self {
        TokenKind::Other(value)
    }
}
