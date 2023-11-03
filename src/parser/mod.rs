//! Abstract syntax tree (AST), nodes and their parse implementations.

use std::iter::Peekable;

mod binary;
mod expr;
mod unary;
mod var;

use crate::{
    lexer::{keywords::others::Other, Span, TokenIterator, TokenKind, Tokenize},
    scanner::Scan,
};

use self::{
    binary::{Binary, BinaryKind},
    expr::{Expr, IntermediateExpr, SimpleExpr},
    unary::{Unary, UnaryKind},
    var::Var,
};

#[derive(Debug, Clone)]
pub struct AsciiMath<'src> {
    iter: Peekable<TokenIterator<'src>>,
}

impl<'s> AsciiMath<'s> {
    pub(crate) fn parse<S: Scan>(input: &'s S) -> Self {
        AsciiMath {
            iter: input.tokenize().peekable(),
        }
    }

    fn parse_simple_expr(&mut self) -> Option<SimpleExpr> {
        let token = self.iter.peek()?;

        if token.is_var() {
            return Some(SimpleExpr::Var(Var::parse(self)?));
        }

        if let TokenKind::Grouping(grouping) = token.kind() {
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

    fn parse_interm_expr(&mut self) -> Option<IntermediateExpr> {
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

        let interm = IntermediateExpr {
            val: s_expr,
            subscript,
            supscript,
        };

        Some(interm)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let interm = self.parse_interm_expr()?;

        if let Some(next_token) = self.iter.peek() {
            if matches!(next_token.kind(), TokenKind::Other(Other::Fraction)) {
                // I/I case
                self.iter.next(); // skip fraction token
                let next_interm = self.parse_interm_expr()?;

                return Some(Expr::Div {
                    numerator: interm,
                    denumerator: next_interm,
                });
            }
        }

        Some(Expr::Interm(interm))
    }
}

impl Iterator for AsciiMath<'_> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expr()
    }
}

#[cfg(test)]
mod tests;
