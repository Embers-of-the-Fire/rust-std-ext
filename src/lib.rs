//! # `std-ext`
//! This crate provides some additions to the standard library api.
//! 
//! ## How to use
//! 
//! In general, the extensions provided by this crate are based on rust's trait system,
//! so you only need to import the provided traits to use the extension methods.
//! 
//! Take `std::option::Option` as an example:
//! 
//! ```rust
//! // import the trait
//! use rs_std_ext::option::OptionExt;
//! 
//! let op: Option<usize> = Some(42);
//! assert_eq! (
//!     // this method is defined by the `OptionExt` trait
//!     op.is_none_or(|x| *x > 0),
//!     true
//! );
//! ```
//! 
//! The layout of the extension trait basically matches the layout of rust std and rust core,
//! with `Ext` as a suffix, e.g. the trait for `std::option::Option` is `rs_std_ext::option::OptionExt`.
//! 


pub mod option;
pub mod result;
pub mod time;
pub mod tuple;
pub mod vec;
pub mod unwrap;

mod compile_warning;
