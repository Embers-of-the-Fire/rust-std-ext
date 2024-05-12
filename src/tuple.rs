//! Extension for typle types.
//!
//! All traits in this module are implemented with tuple no longer than 10.
//!
//! By default, the module only expose implementation for tuple no longer than 5.
//! You can use longer impls by enabling the `long-tuple-impl` feature.
//!
//! Note that the [`TupleConcat`] trait is only implemented for those types that
//! returns a tuple shorter than 10.

/// Zip a tuple with another single value.
///
/// For concating two tuples, see [`TupleConcat`] for more details.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleZip;
///
/// let x = (10u8, 'a');
/// let y = x.zip(-5i32);
///
/// assert_eq!(y, (10u8, 'a', -5i32));
/// ```
pub trait TupleZip<T> {
    type Output;

    /// Zip with another single value.
    fn zip(self, val: T) -> Self::Output;
}

/// Insert a value into a tuple.
///
/// This is internally used behind the [`TupleInsert`] trait for convenience.
///
/// However, if something goes wrong with the trait solver,
/// this trait can also be used explicitly.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleInsertExact;
///
/// let x = (10u8, 'a');
/// let y = <(u8, char) as TupleInsertExact<1, _>>::insert(x, -5i32);
///
/// assert_eq!(y, (10u8, -5i32, 'a'));
/// ```
pub trait TupleInsertExact<const POS: usize, T> {
    type Output;

    fn insert(self, val: T) -> Self::Output;
}

/// Insert a value into a tuple.
///
/// Unlike `TupleInsertExact`, this trait moves the `const POS` to a method,
/// so it can be used with the help of type derivation
/// without explicitly declaring the underlying trait.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleInsert;
///
/// let x = (10u8, 'a');
/// let y = x.insert::<1>(-5i32);
///
/// assert_eq!(y, (10u8, -5i32, 'a'));
/// ```
pub trait TupleInsert<T> {
    fn insert<const POS: usize>(self, val: T) -> Self::Output
    where
        Self: TupleInsertExact<POS, T> + Sized,
    {
        <Self as TupleInsertExact<POS, T>>::insert(self, val)
    }
}

/// Remove a value from a tuple.
///
/// This is internally used behind the [`TupleRemove`] trait for convenience.
///
/// However, if something goes wrong with the trait solver,
/// this trait can also be used explicitly.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleRemoveExact;
///
/// let x = (10u8, 'a', -5i32);
/// let y = <(u8, char, i32) as TupleRemoveExact<1>>::remove(x);
///
/// assert_eq!(y, (10u8, -5i32));
/// ```
pub trait TupleRemoveExact<const POS: usize> {
    type Output;

    fn remove(self) -> Self::Output;
}

/// Remove a value from a tuple.
///
/// Unlike `TupleRemoveExact`, this trait moves the `const POS` to a method,
/// so it can be used with the help of type derivation
/// without explicitly declaring the underlying trait.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleRemove;
///
/// let x = (10u8, 'a', -5i32);
/// let y = x.remove::<1>();
///
/// assert_eq!(y, (10u8, -5i32));
/// ```
pub trait TupleRemove {
    fn remove<const POS: usize>(self) -> Self::Output
    where
        Self: TupleRemoveExact<POS> + Sized,
    {
        <Self as TupleRemoveExact<POS>>::remove(self)
    }
}

/// Concat two tuples.
///
/// This works like [`TupleZip`], but takes another tuple as input.
///
/// **Note:**
/// This trait only has implementations for operations that return tuples of
/// length 5 / 10 (`long-tuple-impl` feature) or less.
/// That means that `(A, B, C, D, E, F, G)` cannot be concated with `(H, I, J, K, L)`.
///
/// ## Example
///
/// ```rust
/// use std_ext::tuple::TupleConcat;
///
/// let x = (10u8, 'a');
/// let y = (-5i32, "foo");
/// let z = x.concat(y);
///
/// assert_eq!(z, (10u8, 'a', -5i32, "foo"));
/// ```
pub trait TupleConcat<T> {
    type Output;

    fn concat(self, other: T) -> Self::Output;
}

mod __generated {
    use paste::paste;

    use super::{
        TupleConcat, TupleInsert, TupleInsertExact, TupleRemove, TupleRemoveExact, TupleZip,
    };

    macro_rules! __impl_tuple_zip {
        ($($ph:ident),+ | $tar:ident) => {
            paste! {
                impl<$($ph),+, $tar> TupleZip<$tar> for ($($ph,)+) {
                    type Output = ($($ph),+, $tar);

                    fn zip(self, val: $tar) -> Self::Output {
                        let ($([< $ph:lower >],)+) = self;
                        ($([< $ph:lower >]),+, val)
                    }
                }
            }
        };
    }

    macro_rules! __impl_tuple_insert {
        ($($ph:ident),+ | $tar:ident) => {
            impl<$($ph),+, $tar> TupleInsert<$tar> for ($($ph,)+) {}
        };
    }

    macro_rules! __impl_tuple_insert_exact {
        ($($ph:ident),+ | $tar:ident at $index:expr => $($res:ident),+) => {
            paste! {
                impl<$($ph),+, $tar> TupleInsertExact<$index, $tar> for ($($ph,)+) {
                    type Output = ($($res,)+);

                    fn insert(self, val: $tar) -> Self::Output {
                        let [< $tar:lower >] = val;
                        let ($([< $ph:lower >],)+) = self;
                        ($([< $res:lower >]),+)
                    }
                }
            }
        };
    }

    macro_rules! __impl_tuple_remove {
        ($($ph:ident),+) => {
            impl<$($ph),+> TupleRemove for ($($ph,)+) {}
        };
    }

    macro_rules! __impl_tuple_remove_exact {
        ($($ph:ident),+ at $index:expr => $($res:ident),+) => {
            paste! {
                impl<$($ph),+> TupleRemoveExact<$index> for ($($ph,)+) {
                    type Output = ($($res,)+);

                    #[allow(unused_variables)]
                    fn remove(self) -> Self::Output {
                        let ($([< $ph:lower >],)+) = self;
                        ($([< $res:lower >],)+)
                    }
                }
            }
        };
    }

    macro_rules! __impl_tuple_concat {
        ($($ph:ident),+ with $($tar:ident),+) => {
            paste! {
                impl<$($ph),+, $($tar),+> TupleConcat<($($tar,)+)> for ($($ph,)+) {
                    type Output = ($($ph),+, $($tar),+);

                    fn concat(self, other: ($($tar,)+)) -> Self::Output {
                        let ($([< $ph:lower >],)+) = self;
                        let ($([< $tar:lower >],)+) = other;
                        ($([< $ph:lower >]),+, $([< $tar:lower >]),+)
                    }
                }
            }
        };
    }

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/tuple_short_impl.rs"
    ));

    #[cfg(feature = "long-tuple-impl")]
    mod __long_tuple_impl {
        use paste::paste;
    
        use super::super::{
            TupleConcat, TupleInsert, TupleInsertExact, TupleRemove, TupleRemoveExact, TupleZip,
        };

        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tuple_long_impl.rs"
        ));
    }
}
