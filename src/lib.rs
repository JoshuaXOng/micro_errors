use std::{any::Any, backtrace::Backtrace, fmt::{Debug, Display}};

#[cfg(test)]
mod tests {
    #[allow(deprecated)]
    use crate::ErrorChain;
    use crate::ErrorLink_;
    use crate::ErrorLinkable;
    use crate::NextLink;
    use std::backtrace::Backtrace;

    #[allow(dead_code)]
    trait X {
        fn letter() -> char;
    }
    impl<T> X for T {
        fn letter() -> char {
            'X'
        }
    }
    struct B;
    impl B {
        fn letter() -> char {
            'b'
        }
    }

    #[test]
    fn test_favour_concrete() {
        assert!(B::letter() == 'b');
    }

    #[test]
    #[allow(non_snake_case)]
    #[allow(deprecated)]
    fn test__chaining_non_error_chain() {
        println!(
            "{}", 
            Err::<(), _>(std::io::Error::other("Underlying error."))
                .map_err(ErrorChain::link_fn("Higher level error."))
                .expect_err("look above")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_link_() {
        println!(
            "{}", 
            Err::<(), _>(std::io::Error::other("Underlying error."))
                .map_err(std::io::Error::link_fn("Higher level error."))
                .expect_err("look above")
        );
    }

    #[test]
    #[allow(deprecated)]
    #[allow(non_snake_case)]
    fn test__chaining_error_chain() {
        println!(
            "{}", 
            Err::<(), _>(ErrorChain::start("Underlying error."))
                .map_err(ErrorChain::link_fn("Higher level error."))
                .expect_err("look above")
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_error_link_() {
        println!(
            "{}", 
            Err::<(), _>(ErrorLink_::new_string("Underlying error."))
                .map_err(ErrorLink_::link_fn("Higher level error."))
                .expect_err("look above")
        );
    }

    impl ErrorLink_<i32> {
        pub fn new_i32(error_number: impl Into<i32>) -> Self {
            Self(error_number.into(), NextLink::None(Backtrace::capture()))
        }
    }
    #[derive(Debug, PartialEq)]
    enum ErrorReasons {
        One,
        Two,
        #[allow(dead_code)]
        Three
    }
    impl ErrorLink_<ErrorReasons> {
        pub fn new_reason(error_reason: ErrorReasons) -> Self {
            Self(error_reason, NextLink::None(Backtrace::capture()))
        }
    }
    impl std::fmt::Display for ErrorReasons {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let as_string = match self {
                ErrorReasons::One => "First reason for underlying error.",
                ErrorReasons::Two => "Second reason for underlying error.",
                ErrorReasons::Three => "Third reason for underlying error.",
            }; 
            write!(f, "{}", as_string)
        }
    }
    impl std::error::Error for ErrorReasons {}

    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_error_link__non_string_payload() {
        println!(
            "{}", 
            Err::<(), _>(ErrorLink_::new_i32(100))
                .map_err(ErrorLink_::link_fn("Higher level error."))
                .expect_err("look above")
        );

        println!(
            "{}", 
            Err::<(), _>(ErrorLink_::new_reason(ErrorReasons::One))
                .map_err(ErrorLink_::link_fn("Higher level error."))
                .expect_err("look above")
        );

        match Err::<(), _>(ErrorLink_::new_reason(ErrorReasons::Two)) {
            Err(error_chain) if error_chain.0 == ErrorReasons::Two => println!("{}", error_chain),
            _ => panic!("look above"),
        }
    }

    #[test]
    #[allow(deprecated)]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_chain_and_non_error_trait() {
        println!(
            "{}", 
            Err::<(), _>(String::from("Underlying error."))
                .map_err(ErrorChain::onboard_fn("Higher level error."))
                .expect_err("look above")
        );

    }
    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_link_and_non_error_trait() {
        println!(
            "{}", 
            Err::<(), _>(String::from("Underlying error."))
                .map_err(String::link_fn("Higher level error."))
                .expect_err("look above")
        );
    }
}

#[derive(Debug)]
#[deprecated(since = "0.3.0", note="use `NextLink` instead")]
pub enum ErrorLink {
    Severed(Backtrace),
    Continued(String, Box<ErrorLink>)
}

#[allow(deprecated)]
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
#[deprecated(since = "0.3.0", note="use `ErrorLink_` instead")]
#[allow(deprecated)]
pub struct ErrorChain<T: Display>(pub T, pub ErrorLink);

#[allow(deprecated)]
impl<T: std::error::Error> From<T> for ErrorChain<String> {
    fn from(value: T) -> Self {
        ErrorChain(value.to_string(), ErrorLink::severed())
    }
}

#[allow(deprecated)]
impl ErrorChain<String> {
    pub fn start(error_message: impl Into<String>) -> Self {
        Self(error_message.into(), ErrorLink::severed())
    }

    pub fn add<T: Display>(error_message: impl Into<String>, current_chain: ErrorChain<T>) -> Self {
        Self::add_fn(error_message)(current_chain)
    }

    pub fn add_fn<T: Display>(error_message: impl Into<String>) -> impl FnOnce(ErrorChain<T>) -> Self { 
        move |current_chain| {
            Self(
                error_message.into(),
                ErrorLink::continued(current_chain.0.to_string(), current_chain.1)
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
                ErrorLink::continued(underlying_error.to_string(), Box::new(ErrorLink::severed())) 
            )
        }
    }

    pub fn link(error_message: impl Into<String>, current_chain: impl Into<Self>) -> Self {
        ErrorChain::link_fn(error_message)(current_chain)
    }

    pub fn link_fn<T: Into<Self>>(error_message: impl Into<String>) -> impl FnOnce(T) -> Self {
        move |current_chain| {
            let current_chain = current_chain.into();
            Self(
                error_message.into(), 
                ErrorLink::continued(current_chain.0, current_chain.1)
            )
        }
    }
}

#[allow(deprecated)]
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

#[derive(Debug)]
pub enum NextLink {
    None(Backtrace),
    Some(Box<ErrorLink_<String>>)
}

#[derive(Debug)]
pub struct ErrorLink_<Payload: Display>(pub Payload, pub NextLink);

impl ErrorLink_<String> {
    pub fn new_string(error_message: impl Into<String>) -> Self {
        Self(error_message.into(), NextLink::None(Backtrace::capture()))
    }

    pub fn link_fn<T: Display>(error_message: impl Into<String>) -> impl FnOnce(ErrorLink_<T>) -> Self {
        move |next_link| {
            let next_link = Box::new(ErrorLink_(next_link.0.to_string(), next_link.1));
            ErrorLink_(error_message.into(), NextLink::Some(next_link))
        }
    }
}

pub trait ErrorLinkable<T, Payload: Display>: Any + Display {
    fn link_fn(error_message: impl Into<Payload>) -> impl FnOnce(T) -> ErrorLink_<Payload>;
}

impl<T: Any + Display> ErrorLinkable<T, String> for T {
    fn link_fn(error_message: impl Into<String>) -> impl FnOnce(T) -> ErrorLink_<String> {
        move |underlying_error| {
            let next_link = Box::new(ErrorLink_(
                String::from(underlying_error.to_string()), 
                NextLink::None(Backtrace::capture())
            ));
            ErrorLink_(error_message.into(), NextLink::Some(next_link))
        }
    }
}

impl<T: Display> Display for ErrorLink_<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error no. 0: {}\n", self.0)?;
        let mut next_link = &self.1;
        for error_number in 1.. {
            next_link = match next_link {
                NextLink::None(end_backtrace) => {
                    write!(
                        f, "Approximate backtrace of error no. {}:\n{end_backtrace}", 
                        error_number - 1
                    )?;
                    break;
                }, 
                NextLink::Some(error_link) => {
                    write!(f, "Error no. {error_number}: {}\n", error_link.0)?;
                    &error_link.1
                },
            }
        }

        Ok(())
    }
}
