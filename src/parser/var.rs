use crate::lexer::{
    keywords::{
        arrows::Arrow, functions::Function, greeks::Greek, logicals::Logical, operators::Operator,
        others::Other, relations::Relation,
    },
    Span, Token, TokenKind,
};

use super::AsciiMath;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VarKind {
    Function(Function),
    Number(String),
    Greek(Greek),
    Variable(String),
    Arrow(Arrow),
    Relation(Relation),
    Logical(Logical),
    Operator(Operator),
    Other(Other),
    Text(String),
}

impl TryFrom<Token<'_>> for VarKind {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let s = match token.kind() {
            TokenKind::Function(f) => Self::Function(f),
            TokenKind::Number => Self::Number(token.as_str().into()),
            TokenKind::Greek(g) => Self::Greek(g),
            TokenKind::Variable => Self::Variable(token.as_str().into()),
            TokenKind::Arrow(a) => Self::Arrow(a),
            TokenKind::Relation(r) => Self::Relation(r),
            TokenKind::Logical(l) => Self::Logical(l),
            TokenKind::Operator(op) => Self::Operator(op),
            TokenKind::Other(other) if token.is_var() => match other {
                Other::Text => Self::Text(token.as_str().into()),
                _ => Self::Other(other),
            },

            _ => return Err(()),
        };

        Ok(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var {
    pub kind: VarKind,
    span: Span,
}

impl Var {
    pub fn parse(parser: &mut AsciiMath) -> Option<Self> {
        let token = parser.iter.next()?;

        if let Ok(var_kind) = VarKind::try_from(token) {
            return Some(Self {
                kind: var_kind,
                span: token.span(),
            });
        }

        None
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
