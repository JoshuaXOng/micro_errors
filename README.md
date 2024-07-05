# micro_errors

Utilities to deal with errors without taking the derive macro approach.

Exposes a struct `ErrorChain` to typically be used within the `Err` variant of `Result`. `ErrorChain` contains a recusive struct `ErrorLink` containing a summary message or a backtrace of the last link.

## Gist

Creating an `ErrorChain` from a non `ErrorChain`.

```rust
Err::<(), _>(std::io::Error::other("not an `ErrorChain`"))
    .map_err(ErrorChain::onboard_fn("that is ok"))
    .expect_err("look above")
```

Creating an `ErrorChain` from an existing `ErrorChain`.

```rust
Err::<(), _>(ErrorChain::start("key glock"))
    .map_err(ErrorChain::add_fn("i dunno"))
    .expect_err("look above")
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
println!(
    "{}", 
    Err::<(), _>(ErrorChain(ErrorReasons::One, ErrorLink::severed()))
        .map_err(|e| ErrorChain::add("food", e))
        .expect_err("look above")
);

match Err::<(), _>(ErrorChain(ErrorReasons::Two, ErrorLink::severed())) {
    Err(error_chain) if error_chain.0 == ErrorReasons::Two => println!("{}", error_chain),
    _ => panic!("look above"),
}
```
