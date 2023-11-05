//! Tokens, lexer and other helper types and functions for tokenization of asciimath input.

mod next_impl;
mod token;

use crate::scanner::{Symbol, Symbols};

pub(crate) use token::{Token, TokenKind};

use self::keywords::{
    accents::Accents,
    arrows::Arrows,
    font_commands::FontCommands,
    functions::Functions,
    greeks::Greeks,
    groupings::Groupings,
    logicals::Logicals,
    operators::Operators,
    others::{Other, Others},
    relations::Relations,
    Keyword, KeywordKind,
};

pub(crate) mod keywords;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Iterator that finds and returns tokens in AsciiMath input.
///
/// In cases where a token is prefix of other token, the longer token is given precedence. For
/// example: 'g' is function g, and 'gamma' is greek letter. In order to correctly identify the
/// greek letter, the longer token must have precedence.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenIterator<'src> {
    src: Vec<Symbol<'src>>,
    curr: usize,
    span: Span,
}

impl<'src> TokenIterator<'src> {
    pub fn tokenize<S>(input: S) -> Self
    where
        S: Into<Symbols<'src>>,
    {
        TokenIterator {
            src: input.into().0,
            ..Default::default()
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.src.get(self.curr) {
                Some(sym) if sym.is_whitespace() => self.curr += 1,
                _ => break,
            }
        }
    }

    /// Identifies a number in AsciiMath input, like 42, 42.24, .3 etc.
    fn lex_number(&self) -> Option<(Token<'src>, usize)> {
        let mut dot_seen = false;

        let start = self.curr;
        let mut curr = self.curr;

        while let Some(sym) = self.src.get(curr) {
            if !sym.is_digit() && !sym.is_dot() {
                break;
            } else if sym.is_dot() {
                if dot_seen {
                    break;
                }

                dot_seen = true;
            }

            curr += 1;
        }

        if start == curr {
            return None;
        }

        let content = Symbol::as_str(self.src.get(start..curr)?)?;

        if matches!(content, ".") {
            // single dot is not a valid number
            return None;
        }

        let kind = TokenKind::Number;

        let span = Span { start, end: curr };
        Some((Token::with_span(content, kind, span), curr))
    }

    /// Lexes a keyword with a given minimum length (or default for the given keyword if that is
    /// longer)
    fn lex_keyword<K>(&self, min_len: usize) -> Option<(Token<'src>, usize)>
    where
        K: Keyword,
    {
        if let Some(sym) = self.src.get(self.curr) {
            if !<K as Keyword>::starts_with(*sym) {
                // none of the corresponding keywords start with the given symbol, so skip parsing
                return None;
            }
        }

        let min_len = K::MIN_LEN.max(min_len);

        let start = self.curr;
        let mut curr = (self.curr + min_len).saturating_sub(1);
        let mut found_at = start;
        let mut keyword = None;

        while let Some(slice) = self.src.get(start..=curr) {
            if slice.last().map(|s| s.is_whitespace()).unwrap_or(false) {
                // token can't contain a whitespace
                break;
            }

            curr += 1;

            let len = slice.len();

            if len > K::MAX_LEN {
                break;
            }

            let slice_str = Symbol::as_str(slice)?;

            if let Some(kind) = <K as Keyword>::get(slice_str) {
                // longer keywords have precedence, otherwise they would not be possible to lex...
                let span = Span { start, end: curr };
                keyword = Some(Token::with_span(slice_str, kind.into(), span));
                found_at = curr;

                match kind.prefix_of() {
                    Some(word_len) => {
                        if len > word_len {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }

        // keyword lexed up to the `found_at` index. Since we check if longer keywords can be
        // lexed, current index is set beyond this point. To make sure we don't overshoot the
        // index, reset to the position where the keyword was actually lexed.
        keyword.map(|k| (k, found_at))
    }

    /// Identifies a greek letter in AsciiMath input, e.g. alpha, beta, pi, Psi etc.
    fn lex_greek(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Greeks>(min_len)
    }

    /// Identifies an arrow in AsciiMath input, e.g. ->, MapsTo, |-> etc.
    fn lex_arrow(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Arrows>(min_len)
    }

    /// Identifies a function in AsciiMath input, e.g. log, sin, cosh etc.
    fn lex_function(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Functions>(min_len)
    }

    fn lex_operator(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Operators>(min_len)
    }

    fn lex_relation(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Relations>(min_len)
    }

    fn lex_logical(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Logicals>(min_len)
    }

    fn lex_grouping(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Groupings>(min_len)
    }

    fn lex_other(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        let (token, cursor) = self.lex_keyword::<Others>(min_len)?;

        let TokenKind::Other(other) = token.kind() else {
            return None;
        };

        if matches!(other, Other::Text | Other::Quote) {
            let (content, new_cursor) = self.lex_text_content(cursor)?;

            let span = Span {
                start: self.curr,
                end: new_cursor,
            };

            let token = Token::with_span(content, TokenKind::Other(Other::Text), span);
            Some((token, new_cursor))
        } else {
            Some((token, cursor))
        }
    }

    fn lex_text_content(&self, cursor: usize) -> Option<(&'src str, usize)> {
        let next_sym = dbg!(self.src.get(cursor))?;

        let closing_str = match next_sym.content {
            "(" => ")",
            _ => "\"",
        };

        let start_idx = if closing_str == ")" {
            cursor + 1
        } else {
            cursor
        };

        let rest = self.src.get(start_idx..)?;
        let closing = rest
            .iter()
            .enumerate()
            .find(|(_, s)| s.content == closing_str)
            .map(|(i, _)| i)?;

        let closing = closing + start_idx;

        let content_syms = self.src.get(start_idx..closing)?;
        let content = Symbol::as_str(content_syms);

        content.map(|c| (c, closing + 1))
    }

    fn lex_accent(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Accents>(min_len)
    }

    fn lex_font_command(&self, min_len: usize) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<FontCommands>(min_len)
    }

    fn lex_variable(&self, _: usize) -> Option<(Token<'src>, usize)> {
        let sym = self.src.get(self.curr)?;

        if !sym.is_letter() {
            return None;
        }

        let span = Span {
            start: self.curr,
            end: self.curr + 1,
        };

        let token = Token::with_span(sym.content, TokenKind::Variable, span);

        Some((token, self.curr + 1))
    }
}

impl<'src> Iterator for TokenIterator<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.src.get(self.curr) {
            Some(_) => {
                next_impl::next_impl!(
                self,
                no_prefix:
                    lex_number;
                prefix:
                    lex_greek,
                    lex_arrow,
                    lex_function,
                    lex_operator,
                    lex_relation,
                    lex_logical,
                    lex_grouping,
                    lex_other,
                    lex_accent,
                    lex_font_command
                    or lex_variable
                )
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
