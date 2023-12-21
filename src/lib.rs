#![warn(missing_docs)]
//! Crate for parsing and rendering of [AsciiMath](http://asciimath.org/).

mod lexer;
mod parser;
mod scanner;

use alemat::MathMl;
use scanner::Symbols;

pub use alemat::Writer;
pub use parser::*;

/// Parse asciimath content into an abstract syntax tree. The whole input is interpreted as a
/// single math block. The result is a list of expressions.
pub fn parse<'s, S>(input: S) -> AsciiMath<'s>
where
    S: Into<Symbols<'s>>,
{
    AsciiMath::parse(input)
}

/// Write an abstract syntax tree into the [`Writer`]. The resulting output is controlled by the
/// implementation of passed in [`Writer`].
///
/// # Errors
///
/// The [`Writer`] may fail to write the mathml. In such case the error defined by the [`Writer`]
/// implementation is returned.
pub fn write_mathml<'w, W>(
    ascii_math: AsciiMath<'_>,
    writer: &'w mut W,
) -> Result<&'w mut W, W::Error>
where
    W: Writer<Buffer = String>,
{
    let mathml = MathMl::from(ascii_math);

    writer.write_mathml(&mathml)?;

    Ok(writer)
}

/// Render the abstract syntax tree into a string of mathml.
pub fn render_mathml(ascii_math: AsciiMath<'_>) -> String {
    let mathml = MathMl::from(ascii_math);
    mathml.render().expect("BufMathMlWriter does not fail.")
}
