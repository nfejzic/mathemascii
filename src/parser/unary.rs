/// Kinds of unary operators in Ascii math.
enum Kind {
    Sqrt,
    Text,
    Abs,
    Floor,
    Ceil,
    Norm,
    Hat,
    Overline,
    Underline,
    Vector,
    Tilde,
    Dot,
    DoubleDot,
    UnderBrace,
    OverBrace,
    Cancel,
    Bold,
    BlackboardBold,
    Calligraphic,
    Typewriter,
    Gothic,
    SansSerif,
}

/// Unary operator in Ascii math.
struct Unary {
    kind: Kind,
}

impl Unary {
    pub(crate) fn parse(/* What should the parameter be? */) -> Self {
        todo!()
    }
}
