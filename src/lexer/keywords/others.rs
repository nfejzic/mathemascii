//! This corresponds to the `Miscellaneous symbols` table at [Ascii math syntax
//! page](http://asciimath.org/#syntax).

use alemat::{
    elements::{Ident, Operator},
    Element,
};

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Other,
    Others,
    "," => Comma,
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
    "..." | "ldots" => LowDots,
    "cdots" => CenterDots,
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

impl From<Other> for Element {
    fn from(value: Other) -> Self {
        match value {
            Other::Comma => Operator::from(",").into(),
            Other::ForwardSlash => Operator::solidus().into(),
            Other::Integral => Operator::integral().into(),
            Other::OIntegral => Operator::circle_integral().into(),
            Other::Partial => Operator::partial_diff().into(),
            Other::Nabla => Operator::nabla().into(),
            Other::PlusMinus => Operator::plus_minus().into(),
            Other::EmptySet => Ident::empty_set().into(),
            Other::Infinity => Ident::infinity().into(),
            Other::Aleph => Ident::aleph().into(),
            Other::Therefore => Operator::therefore().into(),
            Other::Because => Operator::because().into(),
            Other::LowDots => Ident::from("...").into(),
            Other::CenterDots => Ident::from("⋯").into(),
            Other::VerticalDots => Ident::from("⋮").into(),
            Other::DiagonalDots => Ident::from("⋱").into(),
            Other::VerticalBar => Operator::vert_bar().into(),
            Other::VerticalBars => alemat::row![
                Operator::vert_bar(),
                Operator::from(" "),
                Operator::vert_bar()
            ]
            .into(),
            Other::VerticalBarsWide => alemat::row![
                Operator::vert_bar(),
                Operator::from("  "),
                Operator::vert_bar()
            ]
            .into(),
            Other::Angle => Operator::angle().into(),
            Other::Frown => Operator::oparens().into(),
            Other::Triangle => Operator::from("△").into(),
            Other::Diamond => Operator::from("◇").into(),
            Other::Square => Operator::from("□").into(),
            Other::LeftFloor => Operator::lfloor().into(),
            Other::RightFloor => Operator::rfloor().into(),
            Other::LeftCeiling => Operator::lceiling().into(),
            Other::RightCeiling => Operator::rceiling().into(),
            Other::Complex => Ident::set_complex().into(),
            Other::Natural => Ident::set_natural().into(),
            Other::Rational => Ident::set_rational().into(),
            Other::Irrational => Ident::set_irrational().into(),
            Other::Integer => Ident::set_integer().into(),

            // Fallback to string representation
            _ => Operator::from(value.as_ref().to_string()).into(),
        }
    }
}
