# micro_errors

Utilities to deal with errors without taking the derive macro approach.

Exposes a struct `ErrorChain` to typically be used within the `Err` variant of `Result`. `ErrorChain` contains a generic payload and a recusive struct, `ErrorLink`, which in turn contains either a summary message or a backtrace (intended to be of the last link). See below.

```rust
#[derive(Debug)]
pub struct ErrorChain<T: Display>(pub T, pub ErrorLink);
```

For the generic payload as `String`, utilities have been implemented to aid chaining (e.g., `.map_err`'ing).

## Gist

Creating an `ErrorChain` from a non `ErrorChain`.

```rust
pub fn function_1() -> Result<(), ErrorChain<String>> {
    Err::<(), _>(std::io::Error::other("not an `ErrorChain`"))
        .map_err(ErrorChain::onboard_fn("that is ok"))
}
```

Creating an `ErrorChain` from an existing `ErrorChain`.

```rust
pub fn function_2() -> Result<(), ErrorChain<String>> {
    Err::<(), _>(ErrorChain::start("key glock"))
        .map_err(ErrorChain::add_fn("who I be"))
        .map_err(ErrorChain::add_fn("I dunno"))
}
```

Output of displaying/`println!`ing the error being something like below.

```
Error no. 0: I dunno
Error no. 1: who I be
Error no. 2: key glock
Approximate backtrace of error no. 2:
   0: micro_errors::ErrorLink::severed
             at ./src/lib.rs:58:23
   1: micro_errors::ErrorChain<alloc::string::String>::start
             at ./src/lib.rs:76:13
   2: micro_errors::test_crate
             at ./src/lib.rs:14:22
   3: micro_errors::test_crate::{{closure}}
             at ./src/lib.rs:4:16
```

Again, creating an `ErrorChain` from an existing `ErrorChain`.

```rust
#[allow(dead_code)]
#[derive(PartialEq)]
enum ErrorReasons {
    One,
    Two,
    Three
}
impl std::fmt::Display for ErrorReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            ErrorReasons::One => "one",
            ErrorReasons::Two => "two (easter egg)",
            ErrorReasons::Three => "three",
        }; 
        write!(f, "{}", as_string)
    }
}
pub fn function_3() -> Result<(), ErrorChain<String>> {
    match Err::<(), _>(ErrorChain(ErrorReasons::Two, ErrorLink::severed())) {
        Err(error_chain) if error_chain.0 == ErrorReasons::Two => println!("{}", error_chain),
        _ => panic!("look above"),
    }

    Err::<(), _>(ErrorChain(ErrorReasons::One, ErrorLink::severed()))
        .map_err(|e| ErrorChain::add("food", e))
}
```
