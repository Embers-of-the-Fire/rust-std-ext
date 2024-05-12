# `std-ext`

This crate provides some additions to the standard library api.

## How to use

In general, the extensions provided by this crate are based on rust's trait system,
so you only need to import the provided traits to use the extension methods.

Take `std::option::Option` as an example:

```rust
// import the trait
use std_ext::option::OptionExt;

let op: Option<usize> = Some(42);
assert_eq! (
    // this method is defined by the `OptionExt` trait
    op.is_none_or(|x| *x > 0),
    true
);
```

For extended methods, read the documentation on [docs.rs](docs.rs).
The layout of the extension trait basically matches the layout of rust std and rust core,
with `Ext` as a suffix, e.g. the trait for `std::option::Option` is `std_ext::option::OptionExt`.

## License

This crate is licensed under the [MIT License](./LICENSE-MIT) or the [Apache v2.0 License](./LICENSE-APACHE).
