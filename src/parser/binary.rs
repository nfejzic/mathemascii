use alemat::elements::grouping::Style;
use alemat::elements::radicals::Radical;
use alemat::elements::scripted::UnderOver;
use alemat::elements::{Frac, IntoElements};
use alemat::{Attribute, Elements};

use crate::lexer::keywords::accents::Accent;
use crate::lexer::keywords::others::Other;
use crate::lexer::{Span, TokenKind};
use crate::{AsciiMath, Var, VarKind};

use super::expr::SimpleExpr;

/// Kinds of unary operators in Ascii math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryKind {
    /// Fraction binary operation, i.e. `frac(a)(b)` or `a/b`.
    Fraction,

    /// Root binary operation, i.e. `root(a)(b)`.
    Root,

    /// Overset binary operation, i.e. `overset(a)(b)`.
    Overset,

    /// Overset binary operation, i.e. `overset(a)(b)`.
    Underset,

    /// Color binary operation, colors the second expression with the color of the first grouping.
    /// i.e. `color(red)(b)` produces red colored `b` identifier.
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
    /// Returns the [`Span`] occupied by this binary operator expression.
    pub fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn parse(parser: &mut AsciiMath) -> Option<Self> {
        let token = parser.iter.peek()?;
        let binary_kind = BinaryKind::try_from(token.kind()).ok()?;

        let start = token.span().start;

        parser.iter.next(); // skip binary token

        let expr_1 = match binary_kind {
            BinaryKind::Color => Box::new(parser.parse_grouping_as_str()?),
            _ => Box::new(parser.parse_simple_expr()?),
        };

        let expr_2 = Box::new(parser.parse_simple_expr()?);

        let end = expr_2.span().end;

        let binary = Binary {
            kind: binary_kind,
            expr_1,
            expr_2,
            span: Span { start, end },
        };

        Some(binary)
    }
}

impl IntoElements for Binary {
    fn into_elements(self) -> Elements {
        let to_elements = |expr: Box<SimpleExpr>| match *expr {
            SimpleExpr::Grouping(grp) => grp.into_elements(),
            _ => expr.into_elements(),
        };

        match self.kind {
            BinaryKind::Fraction => Frac::builder()
                .num(to_elements(self.expr_1))
                .denom(to_elements(self.expr_2))
                .build()
                .into_elements(),
            BinaryKind::Root => Radical::builder()
                .index(to_elements(self.expr_1))
                .content(to_elements(self.expr_2))
                .build()
                .into_elements(),
            BinaryKind::Overset => UnderOver::builder()
                .expr(to_elements(self.expr_2))
                .over(to_elements(self.expr_1))
                .build()
                .into_elements(),
            BinaryKind::Underset => UnderOver::builder()
                .expr(to_elements(self.expr_2))
                .under(to_elements(self.expr_1))
                .build()
                .into_elements(),
            BinaryKind::Color => {
                let SimpleExpr::Var(var) = *self.expr_1 else {
                    panic!("Expected var with color information");
                };

                let Var {
                    kind: VarKind::Text(color),
                    ..
                } = var
                else {
                    panic!("Expected var with color information");
                };

                let expr = to_elements(self.expr_2);

                Style::from(expr)
                    .with_attr([Attribute::MathColor(color)])
                    .into_elements()
            }
        }
    }
}
