//! Extension for [`std::option::Option`].

/// Extension methods for [`std::option::Option`].
pub trait OptionExt<T> {
    /// Converts an `Option` to a `Result`.
    /// 
    /// Unlike the standard library,
    /// the value of `Option` is treated as `Result::Err` instead of `Result::Ok`.
    /// 
    /// Arguments passed to `err_or` are eagerly evaluated;
    /// if you are passing the result of a function call,
    /// it is recommended to use [`OptionExt::err_or_else`], which is lazily evaluated.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// assert_eq!(Some("err").err_or(0), Err("err"));
    /// assert_eq!(None::<&str>.err_or(0), Ok(0));
    /// ```
    fn err_or<U>(self, ok: U) -> Result<U, T>;

    /// Converts an `Option` to a `Result`.
    /// 
    /// Unlike the standard library,
    /// the value of `Option` is treated as `Result::Err` instead of `Result::Ok`.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// assert_eq!(Some("err").err_or_else(|| 0), Err("err"));
    /// assert_eq!(None::<&str>.err_or_else(|| 0), Ok(0));
    /// ```
    fn err_or_else<U>(self, f: impl FnOnce() -> U) -> Result<U, T>;


    /// Converts an `Option` to a `Result`.
    /// 
    /// Unlike the standard library,
    /// the value of `Option` is treated as `Result::Err` instead of `Result::Ok`.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// assert_eq!(Some("err").err_or_default::<usize>(), Err("err"));
    /// assert_eq!(None::<&str>.err_or_default::<usize>(), Ok(0));
    /// ```
    fn err_or_default<U: Default>(self) -> Result<U, T>;

    /// Returns `true` if the `Option` is a `None` or the value inside of it matches a predicate.
    /// 
    /// This is equivalent to `option.is_some_and(!f)`.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// assert_eq!(None::<usize>.is_none_or(|&i| i == 2), true);
    /// assert_eq!(Some(5).is_none_or(|&i| i == 2), false);
    /// ```
    #[deprecated = "Use `Option::is_none_or` (offered by std) instead."]
    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool;

    /// Returns nothing, consuming the value contained.
    ///
    /// ## Panics
    /// 
    /// Panics if the value is a `Some` with a custom panic message provided by `msg`.
    /// 
    /// ## Example
    /// 
    /// ```rust,should_panic
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// let x: Option<&str> = Some("10");
    /// x.expect_none("fruits are healthy");// panics with `fruits are healthy`
    /// ```
    /// 
    /// ## Recommended Message Style
    /// 
    /// See the documentation for [`Option::expect`][option_expect] for more information.
    /// 
    /// [option_expect]: std::option::Option#method.expect
    fn expect_none(self, msg: &str);

    /// Tap into an `Option` and modify its value.
    /// 
    /// This method captures the `Option` in order not to break the caller-chain.
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use rs_std_ext::option::OptionExt;
    /// 
    /// let mut x: Option<String> = Some("4".into());
    /// assert_eq!(x.tap_mut(|s| s.push('2')), Some("42".into()));
    /// ```
    fn tap_mut(self, f: impl FnOnce(&mut T)) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn err_or<U>(self, ok: U) -> Result<U, T> {
        match self {
            Some(t) => Err(t),
            None => Ok(ok),
        }
    }

    fn err_or_else<U>(self, f: impl FnOnce() -> U) -> Result<U, T> {
        match self {
            Some(t) => Err(t),
            None => Ok(f()),
        }
    }

    fn err_or_default<U: Default>(self) -> Result<U, T> {
        match self {
            Some(t) => Err(t),
            None => Ok(U::default()),
        }
    }

    fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
        match self {
            Some(t) => f(t),
            None => true,
        }
    }

    fn expect_none(self, msg: &str) {
        if self.is_some() {
            panic!("{}", msg)
        }
    }

    fn tap_mut(mut self, f: impl FnOnce(&mut T)) -> Option<T> {
        if let Some(val) = &mut self {
            f(val)
        }

        self
    }
}
