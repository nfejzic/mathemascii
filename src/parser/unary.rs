use alemat::attributes::MathVariant;
use alemat::elements::grouping::{Row, Style};
use alemat::elements::radicals::Radical;
use alemat::elements::scripted::UnderOver;
use alemat::elements::{IntoElements, Num, Operator};
use alemat::{Attribute, Elements};

use crate::lexer::keywords::accents::Accent;
use crate::lexer::keywords::font_commands::FontCommand;
use crate::lexer::keywords::groupings::Grouping;
use crate::lexer::keywords::others::Other;
use crate::lexer::{Span, TokenKind};
use crate::{Var, VarKind};

use super::{expr::SimpleExpr, AsciiMath};

/// Kinds of unary operators in Ascii math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnaryKind {
    // accents:
    Hat,
    Overline,
    Underline,
    Vector,
    Tilde,
    Dot,
    DoubleDot,
    Underbrace,
    Overbrace,
    Cancel,

    // others
    SquareRoot,

    // groupings
    Absolute,
    Floor,
    Ceiling,
    Norm,

    // font commands
    Bold,
    BlackboardBold,
    Calligraphic,
    Typewriter,
    Gothic,
    SansSerif,
}

impl TryFrom<Accent> for UnaryKind {
    type Error = ();

    fn try_from(value: Accent) -> Result<Self, Self::Error> {
        let s = match value {
            Accent::Hat => Self::Hat,
            Accent::Overline => Self::Overline,
            Accent::Underline => Self::Underline,
            Accent::Vector => Self::Vector,
            Accent::Tilde => Self::Tilde,
            Accent::Dot => Self::Dot,
            Accent::DoubleDot => Self::DoubleDot,
            Accent::Underbrace => Self::Underbrace,
            Accent::Overbrace => Self::Overbrace,
            Accent::Cancel => Self::Cancel,

            _ => return Err(()),
        };

        Ok(s)
    }
}

impl TryFrom<Other> for UnaryKind {
    type Error = ();

    fn try_from(value: Other) -> Result<Self, Self::Error> {
        match value {
            Other::SquareRoot => Ok(Self::SquareRoot),

            _ => Err(()),
        }
    }
}

impl TryFrom<Grouping> for UnaryKind {
    type Error = ();

    fn try_from(value: Grouping) -> Result<Self, Self::Error> {
        let val = match value {
            Grouping::Absolute => Self::Absolute,
            Grouping::Floor => Self::Floor,
            Grouping::Ceiling => Self::Ceiling,
            Grouping::NormFn => Self::Norm,
            _ => return Err(()),
        };

        Ok(val)
    }
}

impl TryFrom<FontCommand> for UnaryKind {
    type Error = ();

    fn try_from(value: FontCommand) -> Result<Self, Self::Error> {
        let val = match value {
            FontCommand::Bold => Self::Bold,
            FontCommand::BlackboardBold => Self::BlackboardBold,
            FontCommand::Calligraphic => Self::Calligraphic,
            FontCommand::Typewriter => Self::Typewriter,
            FontCommand::Gothic => Self::Gothic,
            FontCommand::SansSerif => Self::SansSerif,
        };

        Ok(val)
    }
}

impl TryFrom<TokenKind> for UnaryKind {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Accent(accent) => Self::try_from(accent),
            TokenKind::FontCommand(fc) => Self::try_from(fc),
            TokenKind::Other(others) => Self::try_from(others),
            TokenKind::Grouping(grouping) => Self::try_from(grouping),

            _ => Err(()),
        }
    }
}

/// Unary operator in Ascii math.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary {
    /// Kind of unary operator.
    pub(crate) kind: UnaryKind,

    /// Expression that is being operated on.
    pub(crate) expr: Box<SimpleExpr>,

    /// Span occupied by this unary operator expression.
    pub(crate) span: Span,
}

impl Unary {
    /// Returns the [`Span`] occupied by this unary operator expression.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Parses a unary operator expression.
    pub(crate) fn parse(parser: &mut AsciiMath) -> Option<Self> {
        let token = parser.iter.peek()?;
        let unary_kind = UnaryKind::try_from(token.kind()).ok()?;

        let span = token.span();
        let start = span.start;

        parser.iter.next(); // skip unary token

        let expr = parser.parse_simple_expr().unwrap_or_else(|| {
            // empty operator per default
            SimpleExpr::Var(Var {
                kind: VarKind::UnknownOperator(String::default()),
                span: Span {
                    start: span.end,
                    end: span.end,
                },
            })
        });

        let expr = Box::new(expr);

        let end = expr.span().end;

        Some(Unary {
            kind: unary_kind,
            expr,
            span: Span { start, end },
        })
    }
}

impl IntoElements for Unary {
    fn into_elements(self) -> Elements {
        use alemat::children;

        let mut inner = match *self.expr {
            SimpleExpr::Grouping(grp) if grp.is_simple_grp() => {
                grp.ungroup_map(|e| e.into_elements()).collect()
            }
            SimpleExpr::Grouping(grp) => grp.into_elements(),
            _ => self.expr.into_elements(),
        };

        match self.kind {
            UnaryKind::Hat => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::hat())
                .build()]
            .into_elements(),
            UnaryKind::Overline => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::bar())
                .build()]
            .into_elements(),
            UnaryKind::Underline => children![UnderOver::builder()
                .expr(inner)
                .under(Operator::bar())
                .build()]
            .into_elements(),
            UnaryKind::Vector => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::rarrow())
                .build()]
            .into_elements(),
            UnaryKind::Tilde => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::tilde())
                .build()]
            .into_elements(),
            UnaryKind::Dot => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::dot())
                .build()]
            .into_elements(),
            UnaryKind::DoubleDot => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::double_dot())
                .build()]
            .into_elements(),
            UnaryKind::Underbrace => children![UnderOver::builder()
                .expr(inner)
                .under(Operator::ubrace())
                .build()]
            .into_elements(),
            UnaryKind::Overbrace => children![UnderOver::builder()
                .expr(inner)
                .over(Operator::obrace())
                .build()]
            .into_elements(),
            UnaryKind::Cancel => {
                unimplemented!("<menclose> tag is non-standard and not recommended.")
            }
            UnaryKind::SquareRoot => children![Radical::builder()
                .content(inner)
                .index(Num::from(2))
                .build()]
            .into_elements(),
            UnaryKind::Absolute => {
                let mut el = Operator::vert_bar().into_elements();
                el.append(&mut inner);
                el.push(Operator::vert_bar().into());
                el
            }
            UnaryKind::Floor => {
                let mut el = Operator::lfloor().into_elements();
                el.append(&mut inner);
                el.push(Operator::rfloor().into());
                el
            }
            UnaryKind::Ceiling => {
                let mut el = Operator::lceiling().into_elements();
                el.append(&mut inner);
                el.push(Operator::rceiling().into());
                el
            }
            UnaryKind::Norm => {
                let mut el = Operator::norm().into_elements();
                el.append(&mut inner);
                el.push(Operator::norm().into());
                Row::from(el).into_elements()
            }
            UnaryKind::Bold => Style::from(inner)
                .with_attr([Attribute::MathVariant(MathVariant::Bold)])
                .into_elements(),
            UnaryKind::BlackboardBold => Style::from(inner)
                .with_attr([Attribute::MathVariant(MathVariant::DoubleStruck)])
                .into_elements(),
            UnaryKind::Calligraphic => Style::from(inner)
                .with_attr([Attribute::MathVariant(MathVariant::Script)])
                .into_elements(),
            UnaryKind::Typewriter => Style::from(inner)
                // NOTE: not sure if monospace is the one AsciiMath uses here
                .with_attr([Attribute::MathVariant(MathVariant::Monospace)])
                .into_elements(),
            UnaryKind::Gothic => Style::from(inner)
                .with_attr([Attribute::MathVariant(MathVariant::Fraktur)])
                .into_elements(),
            UnaryKind::SansSerif => Style::from(inner)
                .with_attr([Attribute::MathVariant(MathVariant::SansSerif)])
                .into_elements(),
        }
    }
}
