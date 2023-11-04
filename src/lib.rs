mod lexer;
mod parser;
mod scanner;

use parser::AsciiMath;
use scanner::Symbols;

/// Parse asciimath content into an abstract syntax tree.
pub fn parse<'s, S>(input: S) -> AsciiMath<'s>
where
    S: Into<Symbols<'s>>,
{
    AsciiMath::parse(input)
}
