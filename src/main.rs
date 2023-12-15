use alemat::{BufMathMlWriter, Writer};

fn main() {
    let mut args = std::env::args();

    let input = args.nth(1).unwrap();

    let ascii_math = mathemascii::parse(&input);

    let math_ml = mathemascii::write_mathml(ascii_math, &mut BufMathMlWriter::default())
        .map(|w| w.finish())
        .unwrap();

    println!("{math_ml}");
}
