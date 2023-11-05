use crate::lexer::{
    keywords::{accents::Accent, font_commands::FontCommand, groupings::Grouping, others::Other},
    Span, TokenKind,
};

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
            Grouping::Norm => Self::Norm,
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
    pub(crate) kind: UnaryKind,
    pub(crate) expr: Box<SimpleExpr>,
    pub(crate) span: Span,
}

impl Unary {
    pub fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn _parse(parser: &mut AsciiMath) -> Option<Self> {
        let token = parser.iter.next()?;

        let kind = UnaryKind::try_from(token.kind()).ok()?;

        let _expr = match kind {
            UnaryKind::Hat
            | UnaryKind::Overline
            | UnaryKind::Underline
            | UnaryKind::Vector
            | UnaryKind::Tilde
            | UnaryKind::Dot
            | UnaryKind::DoubleDot
            | UnaryKind::Underbrace
            | UnaryKind::Overbrace
            | UnaryKind::Cancel
            | UnaryKind::SquareRoot
            | UnaryKind::Absolute
            | UnaryKind::Floor
            | UnaryKind::Ceiling
            | UnaryKind::Norm => parser.parse_simple_expr()?,

            UnaryKind::Bold => todo!(),
            UnaryKind::BlackboardBold => todo!(),
            UnaryKind::Calligraphic => todo!(),
            UnaryKind::Typewriter => todo!(),
            UnaryKind::Gothic => todo!(),
            UnaryKind::SansSerif => todo!(),
        };

        todo!()
    }
}
