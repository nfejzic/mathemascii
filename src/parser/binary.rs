use crate::lexer::{
    keywords::{accents::Accent, others::Other},
    Span, TokenKind,
};

use super::expr::SimpleExpr;

/// Kinds of unary operators in Ascii math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryKind {
    // others
    Fraction,
    Root,

    // accents
    Overset,
    Underset,
    Color,
}

impl TryFrom<Accent> for BinaryKind {
    type Error = ();

    fn try_from(value: Accent) -> Result<Self, Self::Error> {
        let s = match value {
            Accent::Overset => Self::Overset,
            Accent::Underset => Self::Underset,
            Accent::Color => Self::Color,
            _ => return Err(()),
        };

        Ok(s)
    }
}

impl TryFrom<Other> for BinaryKind {
    type Error = ();

    fn try_from(value: Other) -> Result<Self, Self::Error> {
        match value {
            Other::Fraction => Ok(Self::Fraction),
            Other::Root => Ok(Self::Root),

            _ => Err(()),
        }
    }
}

impl TryFrom<TokenKind> for BinaryKind {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Accent(accent) => Self::try_from(accent),
            TokenKind::Other(others) => Self::try_from(others),

            _ => Err(()),
        }
    }
}

/// Binary operator in Ascii math.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
    pub(crate) kind: BinaryKind,
    pub(crate) expr_1: Box<SimpleExpr>,
    pub(crate) expr_2: Box<SimpleExpr>,
    pub(crate) span: Span,
}

impl Binary {
    pub fn span(&self) -> Span {
        self.span
    }
}
