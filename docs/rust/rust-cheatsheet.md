# Rust lang cheatsheet

## Macros

Export macos to downstream crates

```rust
#[macro_export]
macro_rules! vec { .. }
```

Allow comma in pattern matching

```rust
macro_rules! some_macro {
    ($($element:expr),+ $(,)?) => {
        ..
    }
}
```

Count elements

```rust
#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

// usage
crate::count![@COUNT; $($(element),*)]
```
