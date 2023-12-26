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

use crate::lexer::keywords::{groupings::Grouping, others::Other};
use crate::lexer::{Span, Token, TokenIterator, TokenKind};
use crate::scanner::Symbols;

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

        if let (true, Err(_), Err(_)) = (
            token.kind().is_grouping_open(),
            UnaryKind::try_from(token.kind()),
            BinaryKind::try_from(token.kind()),
        ) {
            let TokenKind::Grouping(grouping) = token.kind() else {
                unreachable!("Must be grouping at this point.");
            };

            let span = token.span();
            let start = span.start;

            let _ = self.iter.next(); // skip open grouping token

            let mut exprs = Vec::default();
            let mut end = span.end;

            let (r_grouping, end) = loop {
                let Some(expr) = self.parse_expr() else {
                    break (Grouping::CloseIgnored, end);
                };

                end = expr.span().end;
                exprs.push(expr);

                // if None next iteration of loop will handle it as `Grouping::CloseIgnored`
                if let TokenKind::Grouping(r_grouping) = self
                    .iter
                    .peek()
                    .map(Token::kind)
                    .unwrap_or(TokenKind::Grouping(Grouping::CloseIgnored))
                {
                    if grouping.matches(r_grouping) {
                        // skip grouping token
                        let e = self.iter.next().map(|t| t.span()).map_or(end, |s| s.end);

                        break (r_grouping, e);
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

        // fallback to var
        Var::parse(self).map(SimpleExpr::Var)
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
            interm: s_expr,
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

                self.iter.next(); // skip '/' token
                let denominator = self.parse_expr()?;

                let start = numerator.span().start;
                let end = denominator.span().end;

                // treat intermediate expressions as parenthesised expressions passed to frac:
                // a_b/c_d == (a_b)/(c_d) == frac{a_b}{c_d}
                let numerator = if numerator.is_scripted() {
                    SimpleExpr::Interm(Box::new(numerator))
                } else {
                    numerator.into_interm_with(|inner| match inner {
                        SimpleExpr::Grouping(grp) => {
                            SimpleExpr::Grouping(grp.ignored_parentheses())
                        }
                        _ => inner,
                    })
                };

                let denominator = if denominator.is_scripted() {
                    SimpleExpr::Interm(Box::new(denominator))
                } else {
                    denominator.into_interm_with(|inner| match inner {
                        SimpleExpr::Grouping(grp) => {
                            SimpleExpr::Grouping(grp.ignored_parentheses())
                        }
                        _ => inner,
                    })
                };

                let binary = Binary {
                    kind: BinaryKind::Fraction,
                    expr_1: Box::new(numerator),
                    expr_2: Box::new(denominator),
                    span: Span { start, end },
                };

                return Some(Expression {
                    interm: SimpleExpr::Binary(binary),
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
