/// Emits a compile-time warning.
///
/// This works like the standard [`std::compile_error`] macro,
/// but emits a warning instead of an error.
///
/// The error is actually a `dead_code`, since rust doesn't
/// support emitting **real** warnings now.
///
/// **Note**
///
/// The macro expands to something like:
///
/// ```rust,ignore
/// #[warn(dead_code)]
/// const WARNING: &str = "<some warning>";
/// ```
///
/// So the ident `WARNING` is reserved.
/// If you want to override it, use the following syntax:
///
/// ```rust,ignore
/// use std_ext::compile_warning;
///
/// compile_warning!(MW: "warning");
/// // this expand to
/// // const MW: &str = "warning";
/// ```
#[macro_export]
macro_rules! compile_warning {
    ($text:expr) => {
        #[warn(dead_code)]
        const WARNING: &str = $text;
    };
    ($ident:ident: $text:expr) => {
        #[warn(dead_code)]
        const $ident: &str = $text;
    };
}
