use std::{backtrace::Backtrace, fmt::{Display, Debug}};

#[test]
fn test_crate() {
    println!(
        "{:#?}", 
        Err::<(), _>(std::io::Error::other("not an `ErrorChain`"))
            .map_err(ErrorChain::onboard_fn("that is ok"))
            .expect_err("look above")
    );

    println!(
        "{}", 
        Err::<(), _>(ErrorChain::start("key glock"))
            .map_err(ErrorChain::add_fn("who I be"))
            .map_err(ErrorChain::add_fn("I dunno"))
            .expect_err("look above")
    );

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
}

#[derive(Debug)]
pub enum ErrorLink {
    Severed(Backtrace),
    Continued(String, Box<ErrorLink>)
}

impl ErrorLink {
    pub fn severed() -> Self {
        Self::Severed(Backtrace::capture())
    }

    pub fn continued(
        error_message: impl Into<String>, 
        next_link: impl Into<Box<ErrorLink>>
    ) -> Self {
        Self::Continued(error_message.into(), next_link.into())
    }
}

#[derive(Debug)]
pub struct ErrorChain<T: Display>(pub T, pub ErrorLink);

impl ErrorChain<String> {
    pub fn start(error_message: impl Into<String>) -> Self {
        Self(
            error_message.into(), 
            ErrorLink::severed()
        )
    }

    pub fn add<T: Display>(error_message: impl Into<String>, current_chain: ErrorChain<T>) -> Self {
        Self::add_fn(error_message)(current_chain)
    }

    pub fn add_fn<T: Display>(error_message: impl Into<String>) -> impl FnOnce(ErrorChain<T>) -> Self { 
        move |current_chain| {
            Self(
                error_message.into(),
                ErrorLink::continued(
                    current_chain.0.to_string(), 
                    current_chain.1
                )
            )
        }
    }

    pub fn onboard<T: Display>(error_message: impl Into<String>, underlying_error: T) -> Self { 
        Self::onboard_fn(error_message)(underlying_error)
    }

    pub fn onboard_fn<T: Display>(error_message: impl Into<String>) -> impl FnOnce(T) -> Self { 
        move |underlying_error| {
            Self(
                error_message.into(),
                ErrorLink::Continued(
                    underlying_error.to_string(), 
                    Box::new(ErrorLink::severed())
                ) 
            )
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ErrorChain<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error no. 0: {}\n", self.0)?;
        let mut error_link = &self.1;
        for error_number in 1.. {
            error_link = match error_link {
                ErrorLink::Severed(end_backtrace) => {
                    write!(
                        f, "Approximate backtrace of error no. {}:\n{end_backtrace}", 
                        error_number - 1
                    )?;
                    break;
                }, 
                ErrorLink::Continued(error_message, error_link) => {
                    write!(f, "Error no. {error_number}: {error_message}\n")?;
                    error_link
                },
            }
        }

        Ok(())
    }
}
