//! This module contains a few quick `unwrap`s
//! for composite `Result`/`Option` types.

use std::fmt::Debug;

/// Extension for `unwrap` methods.
pub trait UnwrapExt {
    type Output;

    /// `unwrap` the value.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::unwrap::UnwrapExt;
    /// 
    /// let val: Result<Option<i32>, &str> = Ok(Some(10));
    /// assert_eq!(
    ///     10,
    ///     val.unwrap_all()
    /// )
    /// ```
    fn unwrap_all(self) -> Self::Output;
}

impl<T, E: Debug> UnwrapExt for Result<Option<T>, E> {
    type Output = T;

    fn unwrap_all(self) -> Self::Output {
        self.unwrap().unwrap()
    }
}

impl<T, E1: Debug, E2: Debug> UnwrapExt for Result<Result<T, E1>, E2> {
    type Output = T;

    fn unwrap_all(self) -> Self::Output {
        self.unwrap().unwrap()
    }
}

impl<T, E: Debug> UnwrapExt for Option<Result<T, E>> {
    type Output = T;

    fn unwrap_all(self) -> Self::Output {
        self.unwrap().unwrap()
    }
}

impl<T> UnwrapExt for Option<Option<T>> {
    type Output = T;

    fn unwrap_all(self) -> Self::Output {
        self.unwrap().unwrap()
    }
}
