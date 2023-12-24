use alemat::elements::IntoElements;

use crate::lexer::{
    keywords::{
        arrows::Arrow, functions::Function, greeks::Greek, logicals::Logical, operators::Operator,
        others::Other, relations::Relation,
    },
    Span, Token, TokenKind,
};

use super::AsciiMath;

/// Kinds of "variables" in Ascii math.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VarKind {
    /// Function identifier, i.e. `f` in `f(x)`.
    Function(Function),

    /// Number, i.e. `1` in `1 + 2`.
    Number(String),

    /// Greek letter, i.e. `alpha` or `pi`.
    Greek(Greek),

    /// Variable identifier, almost any letter i.e. `x` in `f(x)`.
    Variable(String),

    /// Arrow, i.e. `->` or `=>`.
    Arrow(Arrow),

    /// Relation, i.e. `=` or `!=`.
    Relation(Relation),

    /// Logical operator, i.e. `and` or `or`.
    Logical(Logical),

    /// Operator, i.e. `+` or `-`.
    Operator(Operator),

    /// Non-letter symbols not recognized by other keyword definitions fall back to operator, i.e.
    /// `;` or `.`.
    UnknownOperator(String),

    /// Other symbols, like comma (`,`) for example.
    Other(Other),

    /// Text, i.e. `"hello"` or `text(hello)`.
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
            TokenKind::UnknownOperator => Self::UnknownOperator(token.as_str().into()),
            TokenKind::Other(other) => match other {
                Other::Text => Self::Text(token.as_str().into()),
                _ => Self::Other(other),
            },

            _ => return Err(()),
        };

        Ok(s)
    }
}

/// Variable in Ascii math, meaning a symbol that stands on it's own.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var {
    /// Kind of variable.
    pub kind: VarKind,

    /// Span occupied by this variable.
    pub span: Span,
}

impl Var {
    /// Parses a variable from Ascii math input.
    pub(crate) fn parse(parser: &mut AsciiMath) -> Option<Self> {
        let token = parser.iter.next()?;

        if let Ok(var_kind) = VarKind::try_from(token) {
            return Some(Self {
                kind: var_kind,
                span: token.span(),
            });
        }

        None
    }

    /// Returns the [`Span`] occupied by this variable.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Returns `true` if this variable is a comma.
    pub(crate) fn is_comma(&self) -> bool {
        match self.kind {
            VarKind::Other(other) => matches!(other, Other::Comma),
            _ => false,
        }
    }
}

impl IntoElements for Var {
    fn into_elements(self) -> alemat::Elements {
        use alemat::elements::*;

        match self.kind {
            VarKind::Function(i) => Ident::builder().ident(i.as_ref()).build().into_elements(),
            VarKind::Greek(greek) => Ident::from(greek).into_elements(),
            VarKind::Variable(i) => Ident::builder().ident(i).build().into_elements(),

            VarKind::Relation(rel) => Operator::from(rel).into_elements(),
            VarKind::Logical(log) => Operator::from(log).into_elements(),
            VarKind::Operator(op) => Operator::from(op).into_elements(),
            VarKind::Arrow(arrow) => Operator::from(arrow).into_elements(),

            VarKind::Other(ot) => [ot].into_elements(),
            VarKind::Text(txt) => Text::from(txt).into_elements(),
            VarKind::Number(num) => Num::from(num.as_str()).into_elements(),
            VarKind::UnknownOperator(op) => Operator::from(op).into_elements(),
        }
    }
}
