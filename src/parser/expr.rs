use crate::lexer::{keywords::groupings::Grouping, Span};

use super::{binary::Binary, unary::Unary, var::Var};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleExpr {
    Var(Var),

    Grouping {
        left_grouping: Grouping,
        right_grouping: Grouping,
        expr: Vec<Expr>,
        span: Span,
    },

    Unary(Unary),

    Binary(Binary),
}

impl SimpleExpr {
    pub fn span(&self) -> Span {
        match self {
            SimpleExpr::Var(var) => var.span(),
            SimpleExpr::Grouping {
                left_grouping: _,
                right_grouping: _,
                expr: _,
                span,
            } => *span,
            SimpleExpr::Unary(unary) => unary.span(),
            SimpleExpr::Binary(binary) => binary.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntermediateExpr {
    pub(crate) val: SimpleExpr,
    pub(crate) subscript: Option<SimpleExpr>,
    pub(crate) supscript: Option<SimpleExpr>,
}

impl IntermediateExpr {
    pub fn span(&self) -> Span {
        let span = self.val.span();
        let start = span.start;
        let mut end = span.end;

        if let Some(supscript) = &self.supscript {
            end = supscript.span().end;
        }

        Span { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Interm(IntermediateExpr),

    Div {
        numerator: IntermediateExpr,
        denumerator: IntermediateExpr,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Interm(intermediate) => intermediate.span(),

            Expr::Div {
                numerator,
                denumerator,
            } => {
                let start = numerator.span().start;
                let end = denumerator.span().end;
                Span { start, end }
            }
        }
    }
}
