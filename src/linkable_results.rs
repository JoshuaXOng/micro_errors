use std::fmt::Display;
use std::backtrace::Backtrace;
use crate::{ErrorLink_, NextLink};

pub trait LinkableResult1of2<OkVariant> {
    fn me_link<ToPayload: Display>(self, error_payload: impl Into<ToPayload>)
    -> Result<OkVariant, ErrorLink_<ToPayload>>;
    fn me_as_slink(self) -> Result<OkVariant, ErrorLink_<String>>;
}

#[cfg(feature = "nightly")]
impl<OkVariant, ErrorVariant: Display> LinkableResult1of2<OkVariant>
for Result<OkVariant, ErrorVariant> {
    default fn me_link<ToPayload: Display>(self, error_payload: impl Into<ToPayload>)
    -> Result<OkVariant, ErrorLink_<ToPayload>> {
        self.map_err(|e| {
            let next_link = Box::new(ErrorLink_(
                e.to_string(),
                NextLink::None(Backtrace::capture())
            ));
            ErrorLink_(error_payload.into(), NextLink::Some(next_link))
        })
    }

    default fn me_as_slink(self) -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(
            e.to_string(),
            NextLink::None(Backtrace::capture())
        ))
    }
}

impl<OkVariant, FromPayload: Display> LinkableResult1of2<OkVariant>
for Result<OkVariant, ErrorLink_<FromPayload>> {
    fn me_link<ToPayload: Display>(self, error_payload: impl Into<ToPayload>)
    -> Result<OkVariant, ErrorLink_<ToPayload>> {
        self.map_err(|e| {
            let next_link = Box::new(ErrorLink_(e.0.to_string(), e.1));
            ErrorLink_(error_payload.into(), NextLink::Some(next_link))
        })
    }

    fn me_as_slink(self) -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(e.0.to_string(), e.1))
    }
}

pub trait LinkableResult2of2<OkVariant, FromPayload: Display> {
    fn me_as_link<ToPayload: From<FromPayload> + Display>(self)
    -> Result<OkVariant, ErrorLink_<ToPayload>>;
}

impl<OkVariant, FromPayload: Display> LinkableResult2of2<OkVariant, FromPayload>
for Result<OkVariant, ErrorLink_<FromPayload>> {
    fn me_as_link<ToPayload: From<FromPayload> + Display>(self)
    -> Result<OkVariant, ErrorLink_<ToPayload>> {
        self.map_err(|e| ErrorLink_(e.0.into(), e.1))
    }
}

impl<P: Display> From<P> for ErrorLink_<P> {
    fn from(value: P) -> Self {
        ErrorLink_(value, NextLink::None(Backtrace::capture()))
    }
}
