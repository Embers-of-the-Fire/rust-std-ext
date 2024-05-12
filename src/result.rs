//! Extension for [`std::result::Result`].

/// Extension methods for [`std::result::Result`].
pub trait ResultExt<T, E> {
    /// Tap into a `Result` and modify its `Ok` value.
    /// 
    /// This method captures the `Result` in order not to break the caller-chain.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use std_ext::result::ResultExt;
    /// 
    /// let mut x: Result<String, usize> = Ok("4".into());
    /// assert_eq!(x.tap_mut(|s| s.push('2')), Ok("42".into()));
    /// ```
    fn tap_mut(self, f: impl FnOnce(&mut T)) -> Result<T, E>;

    /// Tap into a `Result` and modify its `Err` value.
    /// 
    /// This method captures the `Result` in order not to break the caller-chain.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use std_ext::result::ResultExt;
    /// 
    /// let mut x: Result<String, usize> = Err(40);
    /// assert_eq!(x.tap_err_mut(|s| *s += 2), Err(42));
    /// ```
    fn tap_err_mut(self, f: impl FnOnce(&mut E)) -> Result<T, E>;

    /// Swap the `Ok` and `Err` values of a `Result`.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use std_ext::result::ResultExt;
    /// 
    /// let x: Result<&str, usize> = Ok("foo");
    /// assert_eq!(x.swap(), Err("foo"));
    /// ```
    fn swap(self) -> Result<E, T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn tap_mut(mut self, f: impl FnOnce(&mut T)) -> Result<T, E> {
        if let Ok(val) = &mut self {
            f(val)
        }

        self
    }

    fn tap_err_mut(mut self, f: impl FnOnce(&mut E)) -> Result<T, E> {
        if let Err(e) = &mut self {
            f(e)
        }

        self
    }

    fn swap(self) -> Result<E, T> {
        match self {
            Ok(t) => Err(t),
            Err(e) => Ok(e),
        }
    }
}
