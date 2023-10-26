//! Tokens, lexer and other helper types and functions for tokenization of asciimath input.

mod token;

use crate::scanner::{Scan, Symbol};

use token::{Token, TokenKind};

use self::keywords::{arrows::Arrows, functions::Functions, greek::Greeks, Keyword};

mod keywords;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenIterator<'src> {
    src: Vec<Symbol<'src>>,
    curr: usize,
    span: Span,
}

pub(crate) trait Tokenize {
    fn tokenize(&self) -> TokenIterator;
}

impl<T> Tokenize for T
where
    T: Scan,
{
    fn tokenize(&self) -> TokenIterator {
        TokenIterator {
            src: self.scan(),
            ..Default::default()
        }
    }
}

impl<'src> TokenIterator<'src> {
    fn skip_whitespace(&mut self) {
        loop {
            match self.src.get(self.curr) {
                Some(sym) if sym.is_whitespace() => self.curr += 1,
                _ => break,
            }
        }
    }

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

        let content = Symbol::as_str(self.src.get(start..curr)?)?;
        let kind = TokenKind::Number;

        Some((Token::new(content, kind), curr))
    }

    fn lex_keyword<K>(&self) -> Option<(Token<'src>, usize)>
    where
        K: Keyword,
        K::Kind: std::fmt::Debug,
    {
        if let Some(sym) = self.src.get(self.curr) {
            if !<K as Keyword>::starts_with(*sym) {
                // none of the corresponding keywords start with the given symbol, so skip parsing
                return None;
            }
        }

        let start = self.curr;
        let mut curr = self.curr;
        let mut found_at = start;
        let mut keyword = None;

        while let Some(slice) = self.src.get(start..=curr) {
            curr += 1;

            let len = slice.len();

            if len < K::MIN_LEN {
                continue;
            } else if len > K::MAX_LEN {
                break;
            }

            let slice_str = Symbol::as_str(slice)?;

            if let Some(kind) = <K as Keyword>::get(slice_str) {
                // longer keywords have precedence, otherwise they would not be possible to lex...
                keyword = Some(Token::new(slice_str, kind.into()));
                found_at = curr;
            }
        }

        // keyword lexed up to the `found_at` index. Since we check if longer keywords can be
        // lexed, current index is set beyond this point. To make sure we don't overshoot the
        // index, reset to the position where the keyword was actually lexed.
        keyword.map(|k| (k, found_at))
    }

    fn lex_greek(&mut self) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Greeks>()
    }

    fn lex_arrow(&mut self) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Arrows>()
    }

    fn lex_function(&mut self) -> Option<(Token<'src>, usize)> {
        self.lex_keyword::<Functions>()
    }
}

impl<'src> Iterator for TokenIterator<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let lexing_res = match self.src.get(self.curr) {
            Some(sym) if sym.is_digit() || sym.is_dot() => self.lex_number(),

            Some(_) => self
                .lex_greek()
                .or_else(|| self.lex_arrow())
                .or_else(|| self.lex_function()),

            None => None,
        };

        match lexing_res {
            Some((token, cursor)) => {
                self.curr = dbg!(cursor);
                Some(token)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
