[![Build](https://img.shields.io/github/actions/workflow/status/nfejzic/mathemascii/build.yml?logo=github&label=Build)](https://github.com/nfejzic/mathemascii/actions/workflows/build.yml)
[![CI](https://img.shields.io/github/actions/workflow/status/nfejzic/mathemascii/ci.yml?logo=github&label=CI)](https://github.com/nfejzic/mathemascii/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/docsrs/mathemascii?logo=docs.rs&label=Docs)](https://docs.rs/mathemascii/latest/alemat/)
[![Crates](https://img.shields.io/crates/v/mathemascii?logo=rust)](https://crates.io/crates/mathemascii)

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

## Examples

The code shown in the usage section produces the following output: 

```xml
<math>
<munderover>
  <mo>∑</mo>
  <mrow>
    <mphantom><mo>{</mo></mphantom>
    <mi>i</mi><mo>=</mo><mn>0</mn>
    <mphantom><mo>}</mo></mphantom></mrow>
  <mrow>
    <mphantom><mo>{</mo></mphantom>
    <mi>k</mi><mo>⋅</mo><mn>2</mn>
    <mphantom><mo>}</mo></mphantom>
  </mrow>
</munderover>
<msup>
  <mi>a</mi>
  <mi>k</mi>
</msup>
</math>
```

which produces the following rendering in browsers: 

$$\sum_{n = 0}^{k * 2}{a^k}$$
