use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Function,
    Functions,
    "sin" => Sin,
    "cos" => Cos,
    "tan" => Tan,
    "sec" => Sec,
    "csc" => Csc,
    "cot" => Cot,
    "arcsin" => ArcSin,
    "arccos" => ArcCos,
    "arctan" => ArcTan,
    "sinh" => SinH,
    "cosh" => CosH,
    "tanh" => TanH,
    "sech" => SecH,
    "csch" => CscH,
    "coth" => CotH,
    "exp" => Exp,
    "log" => Log,
    "ln" => Ln,
    "det" => Det,
    "dim" => Dim,
    "mod" => Mod,
    "gcd" => Gcd,
    "lcm" => Lcm,
    "lub" => Lub,
    "glb" => Glb,
    "min" => Min,
    "max" => Max,
    "f" => F,
    "g" => G,
    prefixes:
        Sin => "sinh",
        Cos => "cosh",
        Tan => "tanh",
        Sec => "sech",
        Csc => "csch",
        Cot => "coth",
        G => "gcd" // or glb, but same length so it does not matter
);

impl From<Function> for TokenKind {
    fn from(value: Function) -> Self {
        TokenKind::Function(value)
    }
}
