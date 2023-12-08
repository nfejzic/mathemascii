//! Abstract syntax tree (AST), nodes and their parse implementations.

use std::iter::Peekable;

mod binary;
mod expr;
mod unary;
mod var;

use crate::{
    lexer::{
        keywords::{groupings::Grouping, others::Other},
        Span, TokenIterator, TokenKind,
    },
    scanner::Symbols,
};

use self::{
    binary::{Binary, BinaryKind},
    expr::{Expression, SimpleExpr},
    unary::{Unary, UnaryKind},
    var::Var,
};

#[derive(Debug, Clone)]
pub struct AsciiMath<'src> {
    iter: Peekable<TokenIterator<'src>>,
}

impl<'s> AsciiMath<'s> {
    pub(crate) fn parse<S>(input: S) -> Self
    where
        S: Into<Symbols<'s>>,
    {
        AsciiMath {
            iter: TokenIterator::tokenize(input).peekable(),
        }
    }

    fn parse_simple_expr(&mut self) -> Option<SimpleExpr> {
        let token = self.iter.peek()?;

        if token.is_var() {
            return Some(SimpleExpr::Var(Var::parse(self)?));
        }

        if let (TokenKind::Grouping(grouping), Err(_)) =
            (token.kind(), UnaryKind::try_from(token.kind()))
        {
            let start = token.span().start;

            let _ = self.iter.next(); // skip open grouping token

            let mut exprs = Vec::default();

            let (r_grouping, end) = loop {
                let expr = self.parse_expr()?;

                exprs.push(expr);

                if let TokenKind::Grouping(r_grouping) = self.iter.peek()?.kind() {
                    if grouping.matches(r_grouping) {
                        let t = self.iter.next()?; // skip grouping token

                        break (r_grouping, t.span().end);
                    }
                }
            };

            return Some(SimpleExpr::Grouping {
                left_grouping: grouping,
                right_grouping: r_grouping,
                expr: exprs,
                span: Span { start, end },
            });
        }

        if let Ok(unary_kind) = UnaryKind::try_from(token.kind()) {
            let start = token.span().start;

            self.iter.next(); // skip unary token

            let expr = Box::new(self.parse_simple_expr()?);

            let end = expr.span().end;

            let unary = Unary {
                kind: unary_kind,
                expr,
                span: Span { start, end },
            };

            return Some(SimpleExpr::Unary(unary));
        }

        if let Ok(binary_kind) = BinaryKind::try_from(token.kind()) {
            let start = token.span().start;

            self.iter.next(); // skip binary token

            let expr_1 = Box::new(self.parse_simple_expr()?);
            let expr_2 = Box::new(self.parse_simple_expr()?);

            let end = expr_2.span().end;

            let binary = Binary {
                kind: binary_kind,
                expr_1,
                expr_2,
                span: Span { start, end },
            };

            return Some(SimpleExpr::Binary(binary));
        }

        None
    }

    fn parse_interm_expr(&mut self) -> Option<Expression> {
        let s_expr = self.parse_simple_expr()?;

        let subscript = match self.iter.peek() {
            Some(token) if matches!(token.kind(), TokenKind::Other(Other::Subscript)) => {
                self.iter.next();
                self.parse_simple_expr()
            }
            _ => None,
        };

        let supscript = match self.iter.peek() {
            Some(token) if matches!(token.kind(), TokenKind::Other(Other::Power)) => {
                self.iter.next(); // skip supscript token
                self.parse_simple_expr()
            }
            _ => None,
        };

        let interm = Expression {
            val: s_expr,
            subscript,
            supscript,
        };

        Some(interm)
    }

    fn parse_expr(&mut self) -> Option<Expression> {
        let interm = self.parse_interm_expr()?;

        if let Some(next_token) = self.iter.peek() {
            if matches!(next_token.kind(), TokenKind::Other(Other::ForwardSlash)) {
                // I/I case -> fraction
                let numerator = interm;
                let numer_span = numerator.span();

                self.iter.next(); // skip '/' token
                let denominator = self.parse_expr()?;

                let start = numerator.span().start;
                let end = denominator.span().end;

                // treat intermediate expressions as parenthesised expressions passed to frac:
                // a_b/c_d == (a_b)/(c_d) == frac{a_b}{c_d}
                let numerator = SimpleExpr::Grouping {
                    left_grouping: Grouping::OpenParen,
                    right_grouping: Grouping::CloseParen,
                    expr: vec![numerator],
                    span: numer_span,
                };

                let denominator = SimpleExpr::Grouping {
                    left_grouping: Grouping::OpenParen,
                    right_grouping: Grouping::CloseParen,
                    expr: vec![denominator],
                    span: numerator.span(),
                };

                let binary = Binary {
                    kind: BinaryKind::Fraction,
                    expr_1: Box::new(numerator),
                    expr_2: Box::new(denominator),
                    span: Span { start, end },
                };

                return Some(Expression {
                    val: SimpleExpr::Binary(binary),
                    subscript: None,
                    supscript: None,
                });
            }
        }

        Some(interm)
    }
}

impl Iterator for AsciiMath<'_> {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expr()
    }
}

#[cfg(test)]
mod tests;
