# micro_errors

Utilities to deal with errors without taking the derive macro approach.

Exposes a struct `ErrorLink_` to typically be used within the `Err` variant of `Result`. It contains a generic payload, and further `ErrorLink_`s whose payloads are `String`s. The final link will point to `Backtrace` instead. The `Backtrace` is approximate, especially when linking from a non `ErrorLink_`. Also, remember to set `RUST_BACKTRACE`.

For the generic payload as `String`, utilities have been implemented to aid chaining (e.g., `.map_err`'ing).

## Gist

Creating an `ErrorLink_` from a non `ErrorLink_`.

```rust
use crate::ErrorLinkable;
pub fn function() -> Result<(), ErrorLink_<String>> {
    Err::<(), _>(std::io::Error::other("Underlying error."))
        .map_err(|e| e.as_link())
}
```

Linking an `ErrorLink_` from a non `ErrorLink_`.

```rust
use crate::ErrorLinkable;
pub fn function() -> Result<(), ErrorLink_<String>> {
    Err::<(), _>(std::io::Error::other("Underlying error."))
        .map_err(|e| e.link("Higher level error."))
}
```

Linking an `ErrorLink_` from an existing `ErrorLink_`.

```rust
pub fn function() -> Result<(), ErrorLink_<String>> {
    Err::<(), _>(ErrorLink_::new_string("Underlying error."))
        .map_err(|e| e.link("Higher level error."))
}
```

Output of displaying/`println!`ing the error being something like below.

```
Error no. 0: Higher level error.
Error no. 1: Underlying error.
Approximate backtrace of error no. 1:
   0: micro_errors::ErrorLink_<alloc::string::String>::new_string
             at ./src/lib.rs:270:51
   1: micro_errors::tests::test__chaining_error_link_
             at ./src/lib.rs:72:26
   2: micro_errors::tests::test__chaining_error_link_::{{closure}}
             at ./src/lib.rs:69:36
   3: core::ops::function::FnOnce::call_once
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs:250:5
   4: core::ops::function::FnOnce::call_once
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs:250:5
   5: test::__rust_begin_short_backtrace
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/test/src/lib.rs:625:18
```
