use alemat::elements::IntoElements;

use crate::{
    lexer::{keywords::groupings::Grouping, Span},
    Expression,
};

use super::iter_ext::IterExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupingExpr {
    pub left_grouping: Grouping,
    pub right_grouping: Grouping,
    pub expr: Vec<Expression>,
    pub span: Span,
}

impl GroupingExpr {
    pub fn ignored_parentheses(self) -> Self {
        Self {
            left_grouping: Grouping::OpenIgnored,
            right_grouping: Grouping::CloseIgnored,
            ..self
        }
    }

    /// Checks whether the grouping contains any expressions.
    pub fn is_empty(&self) -> bool {
        self.expr.is_empty()
    }

    /// Returns the number of expressions inside the grouping. Expressions in this case are
    /// separated by commas. That means, multiple expressions are counted as one, if they aren't
    /// separated by a comma.
    ///
    /// For example: `(1 + 2, 3, 4)` has a length of 3, because `1 + 2` is counted as one
    /// expression.
    pub fn len(&self) -> usize {
        self.expr.iter().group_by_commas_ref().count()
    }

    /// Returns an iterator over the expressions between the grouping symbols.
    pub fn iter_inner(&self) -> impl Iterator<Item = &Expression> {
        self.expr.iter()
    }
}

impl IntoElements for GroupingExpr {
    fn into_elements(self) -> alemat::Elements {
        self.expr
            .into_iter()
            .map(IntoElements::into_elements)
            .collect()
    }
}
