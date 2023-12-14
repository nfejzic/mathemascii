macro_rules! test_snap {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            use std::str::FromStr;
            let input = $input;

            let ascii_math = mathemascii::parse(&input);
            let math_ml = mathemascii::render_mathml(ascii_math);

            let formatted = xmlem::Document::from_str(&math_ml)
                .expect(&format!("input: {} is not valid XML.", input))
                .to_string_pretty();

            let snap = format!("{}\n\n{}", input, formatted);

            insta::assert_display_snapshot!(snap);
        }
    };
}

test_snap!(ubrace, "ubrace(1+2)");
test_snap!(ubrace_text, r#"ubrace(1+2+3+4)_("4 terms")"#);
test_snap!(obrace, "obrace(1+2)");
test_snap!(obrace_text, r#"obrace(1+2+3+4)^("4 terms")"#);
test_snap!(color_red, "color(red)(x)");
test_snap!(math_caligraphy, r#"cc "AaBbCc""#);
test_snap!(math_outlined, r#"bbb "AaBbCc""#);
test_snap!(math_fraktur, r#"fr "AaBbCc""#);
test_snap!(matrix_sq, "[[a,b],[c,d]]");
test_snap!(matrix_aug, "[[a,b,|,c],[d,e,|,f]]");
test_snap!(matrix_layout, "{(2x,+,17y,=,23),(x,-,y,=,5):}");
test_snap!(vector, "((a),(b))");
test_snap!(complex_subscripts, "lim_(N->oo) sum_(i=0)^N");
test_snap!(integral, "int_0^1 f(x)dx");
test_snap!(derivative, "f'(x) = dy/dx");
