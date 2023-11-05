use crate::lexer::TokenKind;

use super::{
    binary::Binary,
    expr::{Expr, IntermediateExpr, SimpleExpr},
    unary::Unary,
    var::{Var, VarKind},
    AsciiMath,
};

mod binary;
mod sub_sup_scripts;
mod unary;

macro_rules! test_snap {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            let input = $input;
            let math = $crate::parse(input);

            insta::assert_display_snapshot!(Snapshot((input, math)));
        }
    };
}

use test_snap;

fn indent(num: usize) -> String {
    "| ".repeat(num)
}

struct Snapshot<T>(T);

impl std::fmt::Display for Snapshot<(&str, AsciiMath<'_>)> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (input, math) = &self.0;

        f.write_str(input)?;
        f.write_str("\n\n")?;

        for expr in math.clone() {
            f.write_fmt(format_args!("{}", Snapshot(&expr)))?;
            f.write_str("\n\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<&Vec<Expr>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for expr in self.0 {
            f.write_fmt(format_args!("{}", Snapshot(expr)))?;
            f.write_str("\n\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<&Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Expr::Interm(intermediate) => {
                f.write_str("Interm {\n")?;

                let interm: String = format!("{}", Snapshot(intermediate))
                    .lines()
                    .map(|l| format!("{}{l}", indent(1)))
                    .collect::<Vec<_>>()
                    .join("\n");

                f.write_str(&interm)?;

                f.write_str("\n")?;
                f.write_str("}")?;
            }

            Expr::Div {
                numerator,
                denominator: denumerator,
            } => {
                f.write_str("Div {\n")?;
                f.write_str(&indent(1))?;
                f.write_str("N: \n")?;

                let num: String = format!("{}", Snapshot(numerator))
                    .lines()
                    .map(|l| format!("{}{l}", indent(2)))
                    .collect::<Vec<_>>()
                    .join("\n");

                f.write_str(&num)?;

                f.write_str("\n")?;
                f.write_str(&indent(1))?;
                f.write_str("D: \n")?;

                let den = format!("{}", Snapshot(denumerator))
                    .lines()
                    .map(|l| format!("{}{l}", indent(2)))
                    .collect::<Vec<_>>()
                    .join("\n");

                f.write_str(&den)?;

                f.write_str("\n")?;
                f.write_str("}")?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<&IntermediateExpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("IntermExpr {\n")?;

        let val = format!("{}", Snapshot(&self.0.val))
            .lines()
            .map(|l| format!("{}{l}", indent(1)))
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&val)?;

        if let Some(subscript) = &self.0.subscript {
            f.write_fmt(format_args!("\n{}sub:\n", indent(2)))?;

            let subscript = format!("{}", Snapshot(subscript))
                .lines()
                .map(|l| format!("{}{l}", indent(3)))
                .collect::<Vec<_>>()
                .join("\n");

            f.write_str(&subscript)?;
        }

        if let Some(ref supscript) = self.0.supscript {
            f.write_fmt(format_args!("\n{}sup:\n", indent(2)))?;

            let supscript = format!("{}", Snapshot(supscript))
                .lines()
                .map(|l| format!("{}{l}", indent(3)))
                .collect::<Vec<_>>()
                .join("\n");

            f.write_str(&supscript)?;
        }

        f.write_str("\n")?;
        f.write_str("}")?;

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<&SimpleExpr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SimpleExpr::Var(var) => f.write_fmt(format_args!("{}", Snapshot(var))),
            SimpleExpr::Grouping {
                left_grouping,
                right_grouping,
                expr,
                span: _,
            } => {
                f.write_fmt(format_args!("{:?}\n", left_grouping))?;

                let expr = format!("{}", Snapshot(expr))
                    .lines()
                    .map(|l| format!("{}{l}", indent(1)))
                    .collect::<Vec<_>>()
                    .join("\n");

                f.write_str(expr.trim_end())?;
                f.write_fmt(format_args!("\n{:?}\n", right_grouping))
            }
            SimpleExpr::Unary(unary) => f.write_fmt(format_args!("{}", Snapshot(unary))),
            SimpleExpr::Binary(binary) => f.write_fmt(format_args!("{}", Snapshot(binary))),
        }
    }
}

impl std::fmt::Display for Snapshot<&Var> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.kind))
    }
}

impl std::fmt::Display for Snapshot<VarKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snap = match &self.0 {
            VarKind::Function(f) => format!("{:?}", TokenKind::from(*f)),
            VarKind::Number(num) => num.clone(),
            VarKind::Greek(greek) => format!("{:?}", TokenKind::from(*greek)),
            VarKind::Variable(var) => var.clone(),
            VarKind::Arrow(arr) => format!("{:?}", TokenKind::from(*arr)),
            VarKind::Relation(rel) => format!("{:?}", TokenKind::from(*rel)),
            VarKind::Logical(log) => format!("{:?}", TokenKind::from(*log)),
            VarKind::Operator(op) => format!("{:?}", TokenKind::from(*op)),
            VarKind::Other(ot) => format!("{:?}", TokenKind::from(*ot)),
            VarKind::Text(t) => format!("'{t}'"),
        };

        f.write_str(&snap)
    }
}

impl std::fmt::Display for Snapshot<&Unary> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.kind))?;
        f.write_str("(\n")?;

        let expr = format!("{}", Snapshot(&*self.0.expr))
            .lines()
            .map(|l| format!("{}{l}", indent(1)))
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&expr)?;

        f.write_str("\n)")?;

        Ok(())
    }
}

impl std::fmt::Display for Snapshot<&Binary> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.kind))?;
        f.write_str("(\n")?;

        let expr_1 = format!("{}", Snapshot(&*self.0.expr_1))
            .lines()
            .map(|l| format!("{}{l}", indent(1)))
            .collect::<Vec<_>>()
            .join("\n");

        let expr_2 = format!("{}", Snapshot(&*self.0.expr_2))
            .lines()
            .map(|l| format!("{}{l}", indent(1)))
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&expr_1)?;
        f.write_str(",\n")?;
        f.write_str(&expr_2)?;

        f.write_str("\n)")?;

        Ok(())
    }
}

test_snap!(divide, "a/b");
test_snap!(power, "a^b");
