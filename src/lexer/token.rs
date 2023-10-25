use super::keywords::{arrows::Arrow, functions::Function, greek::Greek};

/// Token identified in the ascii math source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Token<'src> {
    kind: TokenKind,
    content: &'src str,
}

impl Token<'_> {
    pub fn new(content: &str, kind: TokenKind) -> Token<'_> {
        Token { content, kind }
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

    #[default]
    Unimplemented,
}
