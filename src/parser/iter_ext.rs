use std::marker::PhantomData;

use crate::Expression;

/// Extension trait for iterators, useful for grouping expressions.
pub(crate) trait IterExt<'e> {
    /// Groups the list of expressions by commas. Meaning, commas are skipped, and all expressions
    /// until a comma are grouped together in a vector.
    fn group_by_commas(self) -> GroupByCommas<Self::IntoIter>
    where
        Self: IntoIterator<Item = Expression> + Sized,
        Self: Sized;

    /// Same as [`IterExt::group_by_commas`], but returns references to [`Expression`]s instead.
    /// This function is useful when we don't want to consume the implementing type instance.
    fn group_by_commas_ref(self) -> GroupByCommasRef<'e, Self>
    where
        Self: Iterator<Item = &'e Expression> + Sized,
        Self: Sized,
        Self: 'e;
}

pub(crate) struct GroupByCommas<I> {
    inner: I,
}

impl<'e, T> IterExt<'e> for T {
    fn group_by_commas(self) -> GroupByCommas<<Self as IntoIterator>::IntoIter>
    where
        Self: IntoIterator<Item = Expression> + Sized,
        Self: Sized,
    {
        GroupByCommas {
            inner: self.into_iter(),
        }
    }

    fn group_by_commas_ref(self) -> GroupByCommasRef<'e, Self>
    where
        Self: IntoIterator<Item = &'e Expression> + Sized,
        Self: Sized,
        Self: 'e,
    {
        GroupByCommasRef {
            inner: self,
            _mark: PhantomData,
        }
    }
}

impl<I> Iterator for GroupByCommas<I>
where
    I: Iterator<Item = Expression>,
{
    type Item = Vec<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        let exprs = self
            .inner
            .by_ref()
            .take_while(|e| !e.is_comma())
            .collect::<Vec<_>>();

        if exprs.is_empty() {
            None
        } else {
            Some(exprs)
        }
    }
}

pub(crate) struct GroupByCommasRef<'e, I> {
    inner: I,
    _mark: PhantomData<&'e ()>,
}

impl<'e, I> Iterator for GroupByCommasRef<'e, I>
where
    I: Iterator<Item = &'e Expression>,
{
    type Item = Vec<&'e Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        let exprs = self
            .inner
            .by_ref()
            .take_while(|e| !e.is_comma())
            .collect::<Vec<_>>();

        if exprs.is_empty() {
            None
        } else {
            Some(exprs)
        }
    }
}
