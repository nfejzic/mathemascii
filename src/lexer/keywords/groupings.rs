use alemat::{
    elements::{grouping::Phantom, IntoElements, Operator},
    Element, Elements,
};

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Grouping,
    Groupings,
    "(" => OpenParen,
    ")" => CloseParen,
    "[" => OpenBracket,
    "]" => CloseBracket,
    "{" => OpenBrace,
    "}" => CloseBrace,
    "(:" | "langle" | "<<" => LeftAngled,
    ":)" | "rangle" | ">>" => RightAngled,
    "{:" => OpenIgnored,
    ":}" => CloseIgnored,
    "abs" => Absolute,
    "floor" => Floor,
    "ceil" => Ceiling,
    "norm" => Norm,
    prefixes:
        OpenParen => "(:",
        OpenBrace => "{:"
);

impl From<Grouping> for TokenKind {
    fn from(value: Grouping) -> Self {
        TokenKind::Grouping(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GrpCtxt {
    pub(crate) grp: Grouping,
    pub(crate) is_opening: bool,
}

impl From<(Grouping, bool)> for GrpCtxt {
    fn from((grp, is_opening): (Grouping, bool)) -> Self {
        Self { grp, is_opening }
    }
}

impl From<GrpCtxt> for Element {
    fn from(grp: GrpCtxt) -> Self {
        let GrpCtxt { grp, is_opening } = grp;

        match grp {
            Grouping::OpenParen => Operator::lparens().into(),
            Grouping::CloseParen => Operator::rparens().into(),
            Grouping::OpenBracket => Operator::lbracket().into(),
            Grouping::CloseBracket => Operator::rbracket().into(),
            Grouping::OpenBrace => Operator::lbrace().into(),
            Grouping::CloseBrace => Operator::rbrace().into(),
            Grouping::LeftAngled => Operator::langle().into(),
            Grouping::RightAngled => Operator::rangle().into(),
            Grouping::OpenIgnored => Phantom::from(alemat::children![Operator::rbrace()]).into(),
            Grouping::CloseIgnored => Phantom::from(alemat::children![Operator::lbrace()]).into(),
            Grouping::Absolute => Operator::vert_bar().into(),
            Grouping::Floor if is_opening => Operator::lfloor().into(),
            Grouping::Floor => Operator::rfloor().into(),
            Grouping::Ceiling if is_opening => Operator::lceiling().into(),
            Grouping::Ceiling => Operator::rceiling().into(),
            Grouping::Norm => Operator::from("||").into(),
        }
    }
}

impl IntoElements for GrpCtxt {
    fn into_elements(self) -> Elements {
        alemat::children![Element::from(self)].into_elements()
    }
}

impl Grouping {
    pub fn matches(&self, other: Self) -> bool {
        matches!(
            (*self, other),
            (Grouping::OpenParen, Grouping::CloseParen)
                | (Grouping::OpenParen, Grouping::CloseIgnored)
                | (Grouping::CloseParen, Grouping::OpenParen)
                | (Grouping::CloseParen, Grouping::OpenIgnored)
                | (Grouping::OpenBracket, Grouping::CloseBracket)
                | (Grouping::OpenBracket, Grouping::CloseIgnored)
                | (Grouping::CloseBracket, Grouping::OpenBracket)
                | (Grouping::CloseBracket, Grouping::OpenIgnored)
                | (Grouping::OpenBrace, Grouping::CloseBrace)
                | (Grouping::OpenBrace, Grouping::CloseIgnored)
                | (Grouping::CloseBrace, Grouping::OpenBrace)
                | (Grouping::CloseBrace, Grouping::OpenIgnored)
                | (Grouping::LeftAngled, Grouping::RightAngled)
                | (Grouping::LeftAngled, Grouping::CloseIgnored)
                | (Grouping::RightAngled, Grouping::LeftAngled)
                | (Grouping::RightAngled, Grouping::OpenIgnored)
                | (Grouping::OpenIgnored, Grouping::CloseParen)
                | (Grouping::OpenIgnored, Grouping::CloseBracket)
                | (Grouping::OpenIgnored, Grouping::CloseBrace)
                | (Grouping::OpenIgnored, Grouping::RightAngled)
                | (Grouping::OpenIgnored, Grouping::CloseIgnored)
                | (Grouping::CloseIgnored, Grouping::OpenParen)
                | (Grouping::CloseIgnored, Grouping::OpenBracket)
                | (Grouping::CloseIgnored, Grouping::OpenBrace)
                | (Grouping::CloseIgnored, Grouping::LeftAngled)
                | (Grouping::CloseIgnored, Grouping::OpenIgnored)
                | (Grouping::Absolute, Grouping::Absolute)
                | (Grouping::Floor, Grouping::Floor)
                | (Grouping::Ceiling, Grouping::Ceiling)
                | (Grouping::Norm, Grouping::Norm)
        )
    }
}
