# mathemascii - AsciiMath parser

This is a parser for [AsciiMath](http://asciimath.org/) written in
[Rust](https://www.rust-lang.org/).

## Usage

The API of this crate is designed to be as straight forward as possible. Here's
an example: 

```rust
let input = "sum_(i=0)^(k * 2) a^k";

// Creates an iterator over the input that yields expressions
let ascii_math = mathemascii::parse(&input);

// renders the expressions into a single `<math>` block with the default renderer
let math_ml = mathemascii::render_mathml(ascii_math);

println!("{math_ml}");
```

The `mathemascii` uses [`alemat`](https://github.com/nfejzic/alemat) as the
underlying crate for generating the MathMl output. 

There's also the API where you can use custom
[`alemat::Writer`](https://docs.rs/alemat/latest/alemat/trait.Writer.html) for
rendering: 

```rust
let input = "sum_(i=0)^(k * 2) a^k";

// Creates an iterator over the input that yields expressions
let ascii_math = mathemascii::parse(&input);

// create a writer, here we use the default writer.
let mut writer = BufMathMlWriter::default();

// renders the expressions into a single `<math>` block and writes it into the buffer of the writer.
let _ = mathemascii::write_mathml(ascii_math, &mut writer);

// get the inner buffer of the writer
let math_ml = writer.into_inner();

println!("{math_ml}");
```

For convenience, the `mathemascii::write_mathml` function returns the `Result`
with `Result::Ok` containing the mutable reference to `Writer` passed in as the
second parameter. This allows for in-place init of `Writer` and manipulation:

```rust
let input = "sum_(i=0)^(k * 2) a^k";

// Creates an iterator over the input that yields expressions
let ascii_math = mathemascii::parse(&input);

// Write the expressions into a single `<math>` block with the given writer
let math_ml = mathemascii::write_mathml(ascii_math, &mut BufMathMlWriter::default())
    .map(|w| w.finish()) // finish writing and output the buffer
    .unwrap(); // unwrap the result
```

The default writer used is the `BufMathMlWriter` from alemat. This writer uses a
`String` for its buffer, and writing into it is infallible. Therefore, it uses
the [`Infallible`](https://doc.rust-lang.org/core/convert/enum.Infallible.html)
so it's always safe to unwrap the result. If you use a custom `Writer`
implementation, you may want to handle the error case.
