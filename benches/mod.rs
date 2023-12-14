fn main() {
    divan::main();
}

#[divan::bench]
fn parse() {
    let src = "gammag gammag gammag gammag gammag ".repeat(1_000);

    let exprs = mathemascii::parse(divan::black_box(&src));

    let count = exprs.count();
    assert_eq!(count, 10_000);
    divan::black_box(count);
}

#[divan::bench]
fn parse_and_render_1000() {
    let src = "gammag gammag gammag gammag gammag ".repeat(1_000);

    let exprs = mathemascii::parse(divan::black_box(&src));
    let mathml = mathemascii::render_mathml(exprs);

    assert!(!mathml.is_empty());
    divan::black_box(mathml);
}

#[divan::bench]
fn parse_and_render_10_000() {
    let src = "gammag gammag gammag gammag gammag ".repeat(10_000);

    let exprs = mathemascii::parse(divan::black_box(&src));
    let mathml = mathemascii::render_mathml(divan::black_box(exprs));

    assert!(!mathml.is_empty());
    divan::black_box_drop(mathml);
}

#[divan::bench]
fn parse_and_render_20_000() {
    let src = "gammag gammag gammag gammag gammag ".repeat(20_000);

    let exprs = mathemascii::parse(divan::black_box(&src));
    let mathml = mathemascii::render_mathml(divan::black_box(exprs));

    assert!(!mathml.is_empty());
    divan::black_box_drop(mathml);
}
