//! [`std::time`] related extensions.

use std::time::Duration;

/// Extension methods for constructing [`std::time::Duration`] with numbers.
///
/// ## Note
///
/// This trait is not implemented for the `Duration`,
/// for extension methods operating on a `Duration`,
/// see [`crate::time::DurationExt`] for more information.
pub trait DurationNumExt: DurationNumExtFallible {
    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u8.seconds(), Duration::from_secs(10u64));
    /// ```
    ///
    /// ## Panics
    ///
    /// This function panics if the number is too large or negative.
    fn seconds(&self) -> Duration {
        DurationNumExtFallible::seconds(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u8.milliseconds(), Duration::from_millis(10u64));
    /// ```
    ///
    /// ## Panics
    ///
    /// This function panics if the number is too large or negative.
    fn milliseconds(&self) -> Duration {
        DurationNumExtFallible::milliseconds(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u8.microseconds(), Duration::from_micros(10u64));
    /// ```
    ///
    /// ## Panics
    ///
    /// This function panics if the number is too large or negative.
    fn microseconds(&self) -> Duration {
        DurationNumExtFallible::microseconds(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u8.nanoseconds(), Duration::from_nanos(10u64));
    /// ```
    ///
    /// ## Panics
    ///
    /// This function panics if the number is too large or negative.
    fn nanoseconds(&self) -> Duration {
        DurationNumExtFallible::nanoseconds(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.minutes(), Duration::from_secs(10u64 * 60));
    /// ```
    fn minutes(&self) -> Duration {
        DurationNumExtFallible::minutes(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.hours(), Duration::from_secs(10u64 * 60 * 60));
    /// ```
    fn hours(&self) -> Duration {
        DurationNumExtFallible::hours(self).unwrap()
    }

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExt;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.days(), Duration::from_secs(10u64 * 60 * 60 * 24));
    /// ```
    fn days(&self) -> Duration {
        DurationNumExtFallible::days(self).unwrap()
    }
}

/// Extension methods for constructing [`std::time::Duration`] with numbers.
///
/// ## Note
///
/// This trait is not implemented for the `Duration`,
/// for extension methods operating on a `Duration`,
/// see [`crate::time::DurationExt`] for more information.
///
/// ## Warning
///
/// Unlike the `DurationNumExt`, this trait's method returns an
/// `Option<Duration>`.
/// Numbers that may be too large to be represented like [`num::BigInt`]
/// or may be negative like signed integers will implement this trait.
pub trait DurationNumExtFallible {
    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.seconds(), Some(Duration::from_secs(10u64)));
    /// assert!(u128::MAX.seconds().is_none());
    /// ```
    fn seconds(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    ///
    /// assert_eq!(10u128.milliseconds(), Some(Duration::from_millis(10u64)));
    /// assert!(u128::MAX.milliseconds().is_none());
    /// ```
    fn milliseconds(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.microseconds(), Some(Duration::from_micros(10u64)));
    /// assert!(u128::MAX.microseconds().is_none());
    /// ```
    fn microseconds(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.nanoseconds(), Some(Duration::from_nanos(10u64)));
    /// assert!(u128::MAX.nanoseconds().is_none());
    /// ```
    fn nanoseconds(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.minutes(), Some(Duration::from_secs(10u64 * 60)));
    /// ```
    fn minutes(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.hours(), Some(Duration::from_secs(10u64 * 60 * 60)));
    /// ```
    fn hours(&self) -> Option<Duration>;

    /// Create a `Duration`, using the postfix syntax.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std_ext::time::DurationNumExtFallible;
    /// use std::time::Duration;
    ///
    /// assert_eq!(10u128.days(), Some(Duration::from_secs(10u64 * 60 * 60 * 24)));
    /// ```
    fn days(&self) -> Option<Duration>;
}

pub const SECS_PER_MINUTE: u64 = 60;
pub const SECS_PER_HOUR: u64 = 60 * 60;
pub const SECS_PER_DAY: u64 = 60 * 60 * 24;

#[cfg(feature = "crate-num")]
mod __num_impl {
    use std::time::Duration;

    use num::{BigInt, BigUint, ToPrimitive};

    use super::{extfn, DurationNumExtFallible, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE};

    impl DurationNumExtFallible for BigUint {
        #[inline]
        fn seconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn microseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_micros)
        }

        #[inline]
        fn milliseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_millis)
        }

        #[inline]
        fn nanoseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_nanos)
        }

        #[inline]
        fn minutes(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_MINUTE))
                .and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn hours(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_HOUR))
                .and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn days(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_DAY))
                .and_then(extfn::checked_from_secs)
        }
    }

    impl DurationNumExtFallible for BigInt {
        #[inline]
        fn seconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn microseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_micros)
        }

        #[inline]
        fn milliseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_millis)
        }

        #[inline]
        fn nanoseconds(&self) -> Option<Duration> {
            self.to_u64().and_then(extfn::checked_from_nanos)
        }

        #[inline]
        fn minutes(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_MINUTE))
                .and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn hours(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_HOUR))
                .and_then(extfn::checked_from_secs)
        }

        #[inline]
        fn days(&self) -> Option<Duration> {
            self.to_u64()
                .and_then(|v| v.checked_mul(SECS_PER_DAY))
                .and_then(extfn::checked_from_secs)
        }
    }
}

mod __std_impl {
    use std::time::Duration;

    use super::{
        extfn, DurationNumExt, DurationNumExtFallible, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE,
    };

    macro_rules! __impl_non_fallible {
        ($($ty:ty),*) => {
            $(__impl_non_fallible!(@$ty);)*
        };

        (@$ty:ty) => {
            impl DurationNumExt for $ty {
                #[inline]
                fn seconds(&self) -> Duration {
                    Duration::from_secs(*self as u64)
                }

                #[inline]
                fn microseconds(&self) -> Duration {
                    Duration::from_micros(*self as u64)
                }

                #[inline]
                fn milliseconds(&self) -> Duration {
                    Duration::from_millis(*self as u64)
                }

                #[inline]
                fn nanoseconds(&self) -> Duration {
                    Duration::from_nanos(*self as u64)
                }
            }
        };
    }

    __impl_non_fallible!(u8, u16, u32, u64);

    macro_rules! __impl_fallible_int {
        (with-unwrap $($ty:ty),*) => {
            $(__impl_fallible_int!(@$ty);)*
            $(impl DurationNumExt for $ty {})*
        };

        ($($ty:ty),*) => {
            $(__impl_fallible_int!(@$ty);)*
        };

        (@$ty:ty) => {
            impl DurationNumExtFallible for $ty {
                #[inline]
                fn seconds(&self) -> Option<Duration> {
                    (*self).try_into().ok().and_then(extfn::checked_from_secs)
                }

                #[inline]
                fn microseconds(&self) -> Option<Duration> {
                    (*self).try_into().ok().and_then(extfn::checked_from_micros)
                }

                #[inline]
                fn milliseconds(&self) -> Option<Duration> {
                    (*self).try_into().ok().and_then(extfn::checked_from_millis)
                }

                #[inline]
                fn nanoseconds(&self) -> Option<Duration> {
                    (*self).try_into().ok().and_then(extfn::checked_from_nanos)
                }

                #[inline]
                fn minutes(&self) -> Option<Duration> {
                    (*self)
                        .try_into()
                        .ok()
                        .and_then(|e: u64| e.checked_mul(SECS_PER_MINUTE))
                        .and_then(extfn::checked_from_secs)
                }

                #[inline]
                fn hours(&self) -> Option<Duration> {
                    (*self)
                        .try_into()
                        .ok()
                        .and_then(|e: u64| e.checked_mul(SECS_PER_HOUR))
                        .and_then(extfn::checked_from_secs)
                }

                #[inline]
                fn days(&self) -> Option<Duration> {
                    (*self)
                        .try_into()
                        .ok()
                        .and_then(|e: u64| e.checked_mul(SECS_PER_DAY))
                        .and_then(extfn::checked_from_secs)
                }
            }
        };
    }

    __impl_fallible_int!(with-unwrap i8, i16, i32, i64, i128, u128);
    // theoretically infallible
    __impl_fallible_int!(u8, u16, u32, u64);

    macro_rules! __impl_fallible_float {
        ($($ty:ty: $ident:ident),*) => {
            $(__impl_fallible_float!(@$ty, $ident);)*
        };

        (@$ty:ty, $ident:ident) => {
            impl DurationNumExtFallible for $ty {
                #[inline]
                fn seconds(&self) -> Option<Duration> {
                    Duration::$ident(*self).ok()
                }

                #[inline]
                fn microseconds(&self) -> Option<Duration> {
                    Duration::$ident(*self / 1_000_000.0).ok()
                }

                #[inline]
                fn milliseconds(&self) -> Option<Duration> {
                    Duration::$ident(*self / 1_000.0).ok()
                }

                #[inline]
                fn nanoseconds(&self) -> Option<Duration> {
                    Duration::$ident(*self / 1_000_000_000.0).ok()
                }

                #[inline]
                fn minutes(&self) -> Option<Duration> {
                    Duration::$ident(*self * 60.0).ok()
                }

                #[inline]
                fn hours(&self) -> Option<Duration> {
                    Duration::$ident(*self * 3600.0).ok()
                }

                #[inline]
                fn days(&self) -> Option<Duration> {
                    Duration::$ident(*self * 86400.0).ok()
                }
            }

            impl DurationNumExt for $ty {}
        };
    }

    __impl_fallible_float!(f32: try_from_secs_f32, f64: try_from_secs_f64);
}

pub const NANOS_PER_SEC: u32 = 1_000_000_000;
pub const NANOS_PER_MILLI: u32 = 1_000_000;
pub const NANOS_PER_MICRO: u32 = 1_000;
pub const MILLIS_PER_SEC: u64 = 1_000;
pub const MICROS_PER_SEC: u64 = 1_000_000;

/// Static functions for [`std::time::Duration`].
pub mod extfn {
    use std::time::Duration;

    use super::{MICROS_PER_SEC, MILLIS_PER_SEC, NANOS_PER_MICRO, NANOS_PER_MILLI, NANOS_PER_SEC};

    /// A checked version of [`Duration::from_secs`][duration_from_secs].
    ///
    /// [duration_from_secs]: std::time::Duration#method.from_secs
    pub fn checked_from_secs(secs: u64) -> Option<Duration> {
        checked_new(secs, 0)
    }

    /// A checked version of [`Duration::from_millis`][duration_from_millis].
    ///
    /// [duration_from_millis]: std::time::Duration#method.from_millis
    pub fn checked_from_millis(millis: u64) -> Option<Duration> {
        millis
            .checked_div(MILLIS_PER_SEC)
            .zip(
                millis
                    .checked_rem(MILLIS_PER_SEC)
                    .and_then(|r| <u64 as TryInto<u32>>::try_into(r).ok())
                    .and_then(|r| r.checked_mul(NANOS_PER_MILLI)),
            )
            .and_then(|(x, y)| checked_new(x, y))
    }

    /// A checked version of [`Duration::from_micros`][duration_from_micros].
    ///
    /// [duration_from_micros]: std::time::Duration#method.from_micros
    pub fn checked_from_micros(micros: u64) -> Option<Duration> {
        micros
            .checked_div(MICROS_PER_SEC)
            .zip(
                micros
                    .checked_rem(MICROS_PER_SEC)
                    .and_then(|r| <u64 as TryInto<u32>>::try_into(r).ok())
                    .and_then(|r| r.checked_mul(NANOS_PER_MICRO)),
            )
            .and_then(|(x, y)| checked_new(x, y))
    }

    /// A checked version of [`Duration::from_nanos`][duration_from_nanos].
    ///
    /// [duration_from_nanos]: std::time::Duration#method.from_nanos
    pub fn checked_from_nanos(nanos: u64) -> Option<Duration> {
        nanos
            .checked_div(NANOS_PER_SEC as u64)
            .zip(
                nanos
                    .checked_rem(NANOS_PER_SEC as u64)
                    .and_then(|r| <u64 as TryInto<u32>>::try_into(r).ok()),
            )
            .and_then(|(x, y)| checked_new(x, y))
    }

    /// A pre-checked version of [`Duration::new`][duration_new].
    ///
    /// Note that `Duration` does not offer direct construction,
    /// and this method internally calls `Duration::new`, which
    /// might still panics. However, this method uplifts the check,
    /// and theoretically never panics.
    ///
    /// [duration_new]: std::time::Duration#method.new
    pub fn checked_new(secs: u64, nanos: u32) -> Option<Duration> {
        let secs = match secs.checked_add((nanos / NANOS_PER_SEC) as u64) {
            Some(secs) => secs,
            None => return None,
        };
        let nanos = nanos % NANOS_PER_SEC;
        Some(Duration::new(secs, nanos))
    }
}

/// Extension methods for [`std::time::Duration`].
pub trait DurationExt {}
