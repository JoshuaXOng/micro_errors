use std::any::Any;
use std::backtrace::Backtrace;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum NextLink {
    None(Backtrace),
    Some(Box<ErrorLink_<String>>)
}

#[derive(Debug)]
pub struct ErrorLink_<Payload: Display>(pub Payload, pub NextLink);

impl<Payload: Display> ErrorLink_<Payload> {
    pub fn new(error_payload: impl Into<Payload>) -> Self {
        Self(error_payload.into(), NextLink::None(Backtrace::capture()))
    }

    pub fn replace<NewPayload: Display>(
        self, error_payload: impl Into<NewPayload>
    ) -> ErrorLink_<NewPayload>{
        ErrorLink_(error_payload.into(), self.1)
    }

    pub fn link<ToPayload: Display>(self, error_payload: impl Into<ToPayload>) -> ErrorLink_<ToPayload> {
        Self::link_fn(error_payload)(self)
    }

    pub fn link_fn<ToPayload: Display>(error_payload: impl Into<ToPayload>) -> impl FnOnce(Self) -> ErrorLink_<ToPayload> {
        move |underlying_error| {
            let next_link = Box::new(ErrorLink_(underlying_error.0.to_string(), underlying_error.1));
            ErrorLink_(error_payload.into(), NextLink::Some(next_link))
        }
    }

    pub fn as_link<ToPayload: From<Payload> + Display>(self) -> ErrorLink_<ToPayload> {
        ErrorLink_(self.0.into(), self.1)
    }
}

impl ErrorLink_<String> {
    pub fn new_string(error_message: impl Into<String>) -> Self {
        Self(error_message.into(), NextLink::None(Backtrace::capture()))
    }
}

pub trait ErrorLinkable<Self_, Payload: Display>: Any + Display {
    fn link(self, error_payload: impl Into<Payload>) -> ErrorLink_<Payload>;
    fn link_fn(error_payload: impl Into<Payload>) -> impl FnOnce(Self_) -> ErrorLink_<Payload>;
    fn as_link(self) -> ErrorLink_<Payload>;
}

impl<T: Any + Display> ErrorLinkable<T, String> for T {
    fn link(self, error_message: impl Into<String>) -> ErrorLink_<String> {
        Self::link_fn(error_message)(self)
    }

    fn link_fn(error_message: impl Into<String>) -> impl FnOnce(Self) -> ErrorLink_<String> {
        move |underlying_error| {
            let next_link = Box::new(ErrorLink_(
                String::from(underlying_error.to_string()), 
                NextLink::None(Backtrace::capture())
            ));
            ErrorLink_(error_message.into(), NextLink::Some(next_link))
        }
    }

    fn as_link(self) -> ErrorLink_<String> {
        ErrorLink_(
            self.to_string(),
            NextLink::None(Backtrace::capture())
        )
    }
}

impl<Payload: Display> Display for ErrorLink_<Payload> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred.\n")?;
        write!(f, "Link no. 0: {}\n", self.0)?;
        let mut next_link = &self.1;
        for error_number in 1.. {
            next_link = match next_link {
                NextLink::None(end_backtrace) => {
                    write!(
                        f, "Approximate backtrace of link no. {}:\n{end_backtrace}", 
                        error_number - 1
                    )?;
                    break;
                }, 
                NextLink::Some(error_link) => {
                    write!(f, "Link no. {error_number}: {}\n", error_link.0)?;
                    &error_link.1
                },
            }
        }

        Ok(())
    }
}

impl<Payload: Display + Debug> std::error::Error for ErrorLink_<Payload> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.1 {
            NextLink::None(_) => None,
            NextLink::Some(next_link) => Some(next_link)
        }
    }
}
