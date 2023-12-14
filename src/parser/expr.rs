use alemat::{
    elements::{
        grouping::{Phantom, Row},
        scripted::{SubSup, UnderOver},
        ColumnLine, IntoElements, Table, TableAttr, TableCell, TableRow,
    },
    Elements,
};

use crate::{
    lexer::{
        keywords::{functions::Function, groupings::GrpCtxt, operators::Operator, others::Other},
        Span,
    },
    GroupingExpr, UnaryKind, VarKind,
};

use super::{binary::Binary, iter_ext::IterExt, unary::Unary, var::Var};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleExpr {
    Var(Var),

    Grouping(GroupingExpr),

    Unary(Unary),

    Binary(Binary),
}

impl SimpleExpr {
    pub fn span(&self) -> Span {
        match self {
            SimpleExpr::Var(var) => var.span(),
            SimpleExpr::Grouping(GroupingExpr { ref span, .. }) => *span,
            SimpleExpr::Unary(unary) => unary.span(),
            SimpleExpr::Binary(binary) => binary.span(),
        }
    }

    fn is_underover(&self) -> bool {
        match self {
            SimpleExpr::Var(var) => match var.kind {
                VarKind::Operator(op) => matches!(
                    op,
                    Operator::Sum
                        | Operator::Prod
                        | Operator::BigCap
                        | Operator::BigCup
                        | Operator::BigWedge
                ),
                VarKind::Function(func) => matches!(func, Function::Lim),
                _ => false,
            },
            SimpleExpr::Unary(un) => {
                matches!(un.kind, UnaryKind::Underbrace | UnaryKind::Overbrace)
            }
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            SimpleExpr::Grouping(grp) => grp.len(),
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub(crate) val: SimpleExpr,
    pub(crate) subscript: Option<SimpleExpr>,
    pub(crate) supscript: Option<SimpleExpr>,
}

impl Expression {
    pub fn span(&self) -> Span {
        let span = self.val.span();
        let start = span.start;
        let mut end = span.end;

        if let Some(supscript) = &self.supscript {
            end = supscript.span().end;
        }

        Span { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub(crate) fn is_comma(&self) -> bool {
        match self.val {
            SimpleExpr::Var(ref var) => var.is_comma(),
            _ => false,
        }
    }

    fn is_matrix(&self) -> bool {
        let SimpleExpr::Grouping(ref grp) = self.val else {
            return false;
        };

        let mut len = 0;

        for e in &grp.expr {
            if e.is_comma() {
                continue;
            }

            let SimpleExpr::Grouping(ref grp) = e.val else {
                return false;
            };

            // initialize len
            if len == 0 {
                len = grp.expr.iter().group_by_commas_ref().count();
            } else if len != grp.expr.iter().filter(|e| e.is_comma()).count() + 1 {
                return false;
            }
        }

        // each row must have at least one element, and all rows must have exactly the same
        // number of elements
        len != 0
    }

    fn is_vertical_bar(&self) -> bool {
        let SimpleExpr::Var(ref var) = self.val else {
            return false;
        };

        match var.kind {
            VarKind::Other(o) => o == Other::VerticalBar,
            _ => false,
        }
    }

    fn into_matrix(self) -> Elements {
        let SimpleExpr::Grouping(grp) = self.val else {
            panic!("Expected a matrix.");
        };

        let GroupingExpr {
            left_grouping,
            right_grouping,
            expr,
            ..
        } = grp;

        let first_row = expr.get(0).expect("Matrix row expected.");

        // preallocate maximal number of columns
        let num_of_columns = match &first_row.val {
            SimpleExpr::Grouping(grp) => grp.len(),
            _ => unreachable!(),
        };

        let mut column_lines = vec![ColumnLine::Solid; num_of_columns];

        let mut table = Table::default();

        let mut max_len = 0;
        let mut last_was_line = true;
        for row in expr {
            if row.is_comma() {
                continue;
            }

            let SimpleExpr::Grouping(grp) = row.val else {
                unreachable!("Expected a matrix row.");
            };

            let mut table_row = TableRow::default();

            let mut inserted = 0;
            let mut prev_line = false;

            last_was_line &= grp
                .expr
                .last()
                .map(Expression::is_vertical_bar)
                .unwrap_or(false);

            for (curr, mut e) in grp.expr.group_by_commas().enumerate() {
                let is_line = e.len() == 1 && e[0].is_vertical_bar();

                if inserted != curr && !is_line {
                    if !prev_line {
                        column_lines[inserted] = ColumnLine::None;
                    }

                    prev_line = false;
                } else if is_line {
                    if inserted != curr {
                        prev_line = true;
                    }
                    continue;
                }

                let cell = match e.len() {
                    2.. => {
                        let mut r = Row::default();
                        for exp in e {
                            r.add_elements(exp.into_elements());
                        }

                        TableCell::from(r)
                    }
                    1 => {
                        let e = e.pop().expect("Guaranteed to have one element");

                        TableCell::from(e.into_elements())
                    }
                    _ => TableCell::from(Phantom::from(Elements::default())),
                };

                table_row.add_cell(cell);
                inserted = table_row.cells().len() - 1;
            }

            max_len = max_len.max(table_row.cells().len());
            table.add_row(table_row);
        }

        column_lines.truncate(max_len);

        if !last_was_line {
            column_lines[max_len - 1] = ColumnLine::None;
        }

        table.add_attr([TableAttr::ColumnLines(column_lines)]);

        alemat::row![
            GrpCtxt::from((left_grouping, true)),
            table,
            GrpCtxt::from((right_grouping, false))
        ]
        .into_elements()
    }
}

impl IntoElements for Expression {
    fn into_elements(self) -> Elements {
        if self.is_matrix() {
            return self.into_matrix();
        }

        let is_underover = self.val.is_underover();

        let inner = self.val.into_elements();

        if matches!((&self.subscript, &self.supscript), (None, None)) {
            return inner.into_elements();
        }

        let sub = self.subscript.map(|s| match s {
            SimpleExpr::Grouping(grp) => SimpleExpr::Grouping(grp.ignored_parentheses()),
            _ => s,
        });

        let sup = self.supscript.map(|s| match s {
            SimpleExpr::Grouping(grp) => SimpleExpr::Grouping(grp.ignored_parentheses()),
            _ => s,
        });

        if is_underover {
            let builder = UnderOver::builder().expr(inner);

            match (sub, sup) {
                (None, None) => unreachable!(),
                (None, Some(sup)) => builder.over(sup).build(),
                (Some(sub), None) => builder.under(sub).build(),
                (Some(sub), Some(sup)) => builder.under(sub).over(sup).build(),
            }
            .into_elements()
        } else {
            let builder = SubSup::builder().base(inner);

            match (sub, sup) {
                (None, None) => unreachable!(),
                (None, Some(sup)) => builder.supscript(sup).build(),
                (Some(sub), None) => builder.subscript(sub).build(),
                (Some(sub), Some(sup)) => builder.subscript(sub).supscript(sup).build(),
            }
            .into_elements()
        }
    }
}

impl IntoElements for SimpleExpr {
    fn into_elements(self) -> Elements {
        match self {
            SimpleExpr::Var(var) => var.into_elements(),
            SimpleExpr::Grouping(grp) => {
                let lg = GrpCtxt::from((grp.left_grouping, true)).into_elements();
                let mut rg = GrpCtxt::from((grp.right_grouping, false)).into_elements();

                let mut elements = lg;

                for e in grp.expr {
                    elements.append(&mut e.into_elements());
                }

                elements.append(&mut rg);
                elements
            }
            SimpleExpr::Unary(unary) => unary.into_elements(),
            SimpleExpr::Binary(binary) => binary.into_elements(),
        }
    }
}
