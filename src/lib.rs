mod lexer;
mod parser;
mod scanner;

use parser::AsciiMath;
use scanner::Scan;

/// Parse asciimath content into an abstract syntax tree.
pub fn parse<S: Scan>(input: &S) -> AsciiMath {
    AsciiMath::parse(input)
}
