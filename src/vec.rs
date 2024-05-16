//! [`std::vec::Vec`] related extensions.

use std::cmp::Ordering;

/// Extension methods for [`std::vec::Vec`].
pub trait VecExt<T> {
    /// Same behaviour as [`Vec::sort`][vec_sort],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort]: std::vec::Vec#method.sort
    fn sorted(self) -> Self
    where
        T: Ord;

    /// Same behaviour as [`Vec::sort_by`][vec_sort_by],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_by]: std::vec::Vec#method.sort_by
    fn sorted_by(self, compare: impl FnMut(&T, &T) -> Ordering) -> Self;

    /// Same behaviour as [`Vec::sort_by_key`][vec_sort_by_key],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_by_key]: std::vec::Vec#method.sort_by_key
    fn sorted_by_key<K>(self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord;

    /// Same behaviour as [`Vec::sort_by_cached_key`][vec_sort_by_cached_key],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_by_cached_key]: std::vec::Vec#method.sort_by_cached_key
    fn sorted_by_cached_key<K>(self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord;

    /// Same behaviour as [`Vec::sort_unstable`][vec_sort_unstable],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_unstable]: std::vec::Vec#method.sort_unstable
    fn sorted_unstable(self) -> Self
    where
        T: Ord;

    /// Same behaviour as [`Vec::sort_unstable_by`][vec_sort_unstable_by],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_unstable_by]: std::vec::Vec#method.sort_unstable_by
    fn sorted_unstable_by(self, compare: impl FnMut(&T, &T) -> Ordering) -> Self;

    /// Same behaviour as [`Vec::sort_unstable_by_key`][vec_sort_unstable_by_key],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_sort_unstable_by_key]: std::vec::Vec#method.sort_unstable_by_key
    fn sorted_unstable_by_key<K>(self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord;

    /// Same behaviour as [`Vec::reverse`][vec_reverse],
    /// but returns itself in a form consistent with chained calls.
    ///
    /// [vec_reverse]: std::vec::Vec#method.reverse
    fn reversed(self) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.sort();
        self
    }

    fn sorted_by(mut self, compare: impl FnMut(&T, &T) -> Ordering) -> Self {
        self.sort_by(compare);
        self
    }

    fn sorted_by_key<K>(mut self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord,
    {
        self.sort_by_key(f);
        self
    }

    fn sorted_by_cached_key<K>(mut self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord,
    {
        self.sort_by_cached_key(f);
        self
    }

    fn sorted_unstable(mut self) -> Self
    where
        T: Ord,
    {
        self.sort_unstable();
        self
    }

    fn sorted_unstable_by(mut self, compare: impl FnMut(&T, &T) -> Ordering) -> Self {
        self.sort_unstable_by(compare);
        self
    }

    fn sorted_unstable_by_key<K>(mut self, f: impl FnMut(&T) -> K) -> Self
    where
        K: Ord,
    {
        self.sort_unstable_by_key(f);
        self
    }

    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}
