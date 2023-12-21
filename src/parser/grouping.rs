use alemat::elements::IntoElements;

use crate::{
    lexer::{keywords::groupings::Grouping, Span},
    Expression,
};

use super::iter_ext::IterExt;

/// AsciiMath grouping expression - any number of [`Expression`]s grouped between two grouping
/// symbols, such as parentheses, brackets, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupingExpr {
    /// The opening grouping symbol.
    pub left_grouping: Grouping,

    /// The opening grouping symbol.
    pub right_grouping: Grouping,

    /// The expressions inside the grouping, separated by comma.
    pub expr: Vec<Expression>,

    /// The span of the grouping.
    pub span: Span,
}

impl GroupingExpr {
    /// Replaces the grouping symbols with ignored parentheses.
    /// (See [`Grouping::OpenIgnored`] and [`Grouping::CloseIgnored`])
    pub fn ignored_parentheses(self) -> Self {
        Self {
            left_grouping: Grouping::OpenIgnored,
            right_grouping: Grouping::CloseIgnored,
            ..self
        }
    }

    /// Returns the group of expressions inside the grouping without the grouping symbols.
    pub fn ungroup(self) -> Vec<Expression> {
        self.expr
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
