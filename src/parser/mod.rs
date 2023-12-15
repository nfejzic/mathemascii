//! Abstract syntax tree (AST), nodes and their parse implementations.

use std::iter::Peekable;

mod binary;
mod expr;
mod grouping;
mod iter_ext;
mod unary;
mod var;

use alemat::MathMl;
pub use binary::*;
pub use expr::*;
pub use grouping::*;
pub use unary::*;
pub use var::*;

use crate::{
    lexer::{
        keywords::{groupings::Grouping, others::Other},
        Span, TokenIterator, TokenKind,
    },
    scanner::Symbols,
};

/// Iterator that parses AsciiMath input and yields [`Expression`]s.
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

    fn parse_grouping_as_str(&mut self) -> Option<SimpleExpr> {
        let mut content = String::default();
        let token = self.iter.next()?;
        let start = token.span().start;
        let end;

        let TokenKind::Grouping(opening) = token.kind() else {
            return None;
        };

        loop {
            let token = self.iter.next()?;

            if let TokenKind::Grouping(closing) = token.kind() {
                if opening.matches(closing) {
                    end = token.span().end;
                    break;
                }
            } else {
                content.push_str(token.as_str());
            }
        }

        let var = Var {
            kind: VarKind::Text(content),
            span: Span { start, end },
        };

        Some(SimpleExpr::Var(var))
    }

    fn parse_simple_expr(&mut self) -> Option<SimpleExpr> {
        let token = self.iter.peek()?;

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

            return Some(SimpleExpr::Grouping(GroupingExpr {
                left_grouping: grouping,
                right_grouping: r_grouping,
                expr: exprs,
                span: Span { start, end },
            }));
        }

        if UnaryKind::try_from(token.kind()).is_ok() {
            return Unary::parse(self).map(SimpleExpr::Unary);
        }

        if BinaryKind::try_from(token.kind()).is_ok() {
            return Binary::parse(self).map(SimpleExpr::Binary);
        }

        if token.is_var() {
            return Var::parse(self).map(SimpleExpr::Var);
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
                let numerator = SimpleExpr::Grouping(GroupingExpr {
                    left_grouping: Grouping::OpenParen,
                    right_grouping: Grouping::CloseParen,
                    expr: vec![numerator],
                    span: numer_span,
                });

                let denominator = SimpleExpr::Grouping(GroupingExpr {
                    left_grouping: Grouping::OpenParen,
                    right_grouping: Grouping::CloseParen,
                    expr: vec![denominator],
                    span: numerator.span(),
                });

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

impl From<AsciiMath<'_>> for MathMl {
    fn from(value: AsciiMath<'_>) -> Self {
        let mut mathml = MathMl::default();

        for expr in value {
            mathml.append_content(expr);
        }

        mathml
    }
}

#[cfg(test)]
mod tests;
