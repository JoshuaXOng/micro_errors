use std::fmt::Display;
use std::backtrace::Backtrace;

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
        write!(f, "Link no. 0: {}\n", self.0)?;
        let mut error_link = &self.1;
        for error_number in 1.. {
            error_link = match error_link {
                ErrorLink::Severed(end_backtrace) => {
                    write!(
                        f, "Approximate backtrace of link no. {}:\n{end_backtrace}", 
                        error_number - 1
                    )?;
                    break;
                }, 
                ErrorLink::Continued(error_message, error_link) => {
                    write!(f, "Link no. {error_number}: {error_message}\n")?;
                    error_link
                },
            }
        }

        Ok(())
    }
}
