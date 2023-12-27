use super::{
    keywords::{
        accents::Accent, arrows::Arrow, font_commands::FontCommand, functions::Function,
        greeks::Greek, groupings::Grouping, logicals::Logical, operators::Operator, others::Other,
        relations::Relation,
    },
    Span,
};

/// Token identified in the ascii math source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Token<'src> {
    kind: TokenKind,
    content: &'src str,
    span: Span,
}

impl Token<'_> {
    pub fn with_span(content: &str, kind: TokenKind, span: Span) -> Token<'_> {
        Token {
            content,
            kind,
            span,
        }
    }

    #[allow(dead_code)] // is used in testing
    pub fn span(&self) -> Span {
        self.span
    }

    #[allow(dead_code)] // is used in testing
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    #[allow(dead_code)] // is used in testing
    pub fn as_str(&self) -> &str {
        self.content
    }
}

/// Kind of token identified in ascii math input.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TokenKind {
    /// Number literal.
    Number,

    /// Greek letter, e.g. pi, alpha etc.
    Greek(Greek),

    /// Various arrows, for example |->
    Arrow(Arrow),

    /// Standard functions, eg. sin, cos etc.
    Function(Function),

    /// Standard operators, e.g. +, -, *, |>< etc.
    Operator(Operator),

    /// Symbols that are not letters, and aren't explicitely defined in AsciiMath grammar fall back
    /// to unknown operators.
    UnknownOperator,

    /// Relations in maths, e.g. =, !=, <, <= etc.
    Relation(Relation),

    /// Logic symbols, e.g. and, or, for all etc.
    Logical(Logical),

    /// Grouping brackets in ascii math, e.g. (, {, [, (: etc.
    Grouping(Grouping),

    /// Other (Miscellaneous) symbols of ascii math.
    Other(Other),

    /// Ascii math keywords for accents, e.g. hat, bar, ubrace etc.
    Accent(Accent),

    /// Ascii math font commands such as bb, cc etc.
    FontCommand(FontCommand),

    /// Other Ascii symbol that was not recognized as a token.
    Variable,

    #[default]
    Unimplemented,
}

impl TokenKind {
    pub fn is_grouping_open(&self) -> bool {
        let TokenKind::Grouping(grp) = self else {
            return false;
        };

        !matches!(
            grp,
            Grouping::CloseParen
                | Grouping::CloseBracket
                | Grouping::CloseBrace
                | Grouping::RightAngled
                | Grouping::CloseIgnored
        )
    }
}
