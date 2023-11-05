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
