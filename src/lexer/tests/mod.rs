use crate::lexer::{token::Token, Tokenize};

use super::Span;

mod accents;
mod arrows;
mod functions;
mod greeks;
mod groupings;
mod logicals;
mod numbers;
mod operators;
mod others;
mod relations;

macro_rules! test_snap {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            let input = $input;
            let tokens: Vec<_> = input.tokenize().collect();
            insta::assert_display_snapshot!(Snapshot((input, tokens)));
        }
    };
}

use test_snap;

struct Snapshot<T>(T);

impl std::fmt::Display for Snapshot<Token<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = self.0;

        let indent = " ".repeat(token.span().start);

        f.write_str(&indent)?;
        f.write_str(token.as_str())?;
        f.write_str("\n")?;
        f.write_str(&indent)?;
        f.write_str(&("^".repeat(token.as_str().len())))?;
        f.write_str(" -> ")?;
        f.write_fmt(format_args!("{:?}", token.kind()))?;
        f.write_str(" at: ")?;
        f.write_fmt(format_args!("{}", Snapshot(token.span())))
    }
}

impl std::fmt::Display for Snapshot<(&str, Vec<Token<'_>>)> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (input, tokens) = &self.0;

        // reconstruct input:
        f.write_str(input)?;
        f.write_str("\n\n")?;

        for token in tokens.iter() {
            Snapshot(*token).fmt(f)?;
            f.write_str("\n\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<Span> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.0.start, self.0.end))
    }
}

test_snap!(skip_whitespace, "   alpha   24.42");

#[test]
fn perf() {
    let src = "gammag gammag gammag gammag gammag ".repeat(1_000);
    let src = src.as_str();

    let tokens = src.tokenize();

    assert_eq!(tokens.count(), 10_000);
}
