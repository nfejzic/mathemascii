//! Constants containing all the keywords defined in the asciimath grammar.

use crate::scanner::Symbol;

use super::token::TokenKind;

mod macros;

pub mod arrows;
pub mod functions;
pub mod greek;

pub(crate) trait KeywordKind: Into<TokenKind> + Copy {
    fn prefix_of(&self) -> Option<usize>;
}

pub(crate) trait Keyword {
    const MAX_LEN: usize;
    const MIN_LEN: usize;

    type Kind: KeywordKind;

    fn get(key: &str) -> Option<Self::Kind>;
    fn starts_with(symbol: Symbol<'_>) -> bool;
}
