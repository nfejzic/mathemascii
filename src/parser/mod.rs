//! Abstract syntax tree (AST), nodes and their parse implementations.

use crate::{
    lexer::{TokenIterator, Tokenize},
    scanner::Scan,
};

/// The AST of an AsciiMath expression.
mod unary;
mod var;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiMath<'src> {
    iter: TokenIterator<'src>,
}

struct Var;

impl<'s> AsciiMath<'s> {
    pub(crate) fn parse<S: Scan>(input: &'s S) -> Self {
        AsciiMath {
            iter: input.tokenize(),
        }
    }

    fn parse_var(&mut self) -> Option<Var> {
        None
    }

    // fn parse_simple_expr(&mut self) -> Option<SimpleExpr> {
    //     None
    // }
    //
    // fn parse_interm_expr(&mut self) -> Option<IntermediateExpr> {
    //     None
    // }
    //
    // fn parse_expr(&mut self) -> Option<Expression> {
    //     None
    // }
}

impl Iterator for AsciiMath<'_> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
