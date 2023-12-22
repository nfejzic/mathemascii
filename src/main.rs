use alemat::{BufMathMlWriter, DisplayAttr, MathMl, MathMlAttr, Writer};

fn main() {
    let mut args = std::env::args().peekable();
    args.next(); // skip program name

    let is_block = match args.peek() {
        Some(arg) => {
            matches!(arg.as_str(), "--block" | "-b")
        }
        None => false,
    };

    if is_block {
        // skip blocks argument
        args.next();
    }

    let input = args.next().unwrap();

    let ascii_math = mathemascii::parse(&input);

    let mut math = MathMl::from(ascii_math);

    if is_block {
        math.add_attr(MathMlAttr::Display(DisplayAttr::Block));
    }

    let math_ml = math
        .write(&mut BufMathMlWriter::default())
        .map(Writer::finish)
        .unwrap();

    println!("{math_ml}");
}
