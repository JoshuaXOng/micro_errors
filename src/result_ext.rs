use std::fmt::Display;
use std::backtrace::Backtrace;
use crate::{ErrorLink_, NextLink};

#[cfg(feature = "nightly")]
pub trait ResultExt<OkVariant, ToPayload: Display> {
    fn me_l(self, error_payload: impl Into<ToPayload>)
    -> Result<OkVariant, ErrorLink_<ToPayload>>;
    fn me_al(self) -> Result<OkVariant, ErrorLink_<ToPayload>>;
}

#[cfg(feature = "nightly")]
impl<OkVariant, ErrorVariant: Display> ResultExt<OkVariant, String> 
for Result<OkVariant, ErrorVariant> {
    default fn me_l(self, error_payload: impl Into<String>)
    -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| {
            let next_link = Box::new(ErrorLink_(
                e.to_string(), 
                NextLink::None(Backtrace::capture()))
            );
            ErrorLink_(error_payload.into(), NextLink::Some(next_link))
        })
    }

    default fn me_al(self) -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(
            e.to_string(),
            NextLink::None(Backtrace::capture())
        ))
    }
}

#[cfg(feature = "nightly")]
impl<OkVariant> ResultExt<OkVariant, String> 
for Result<OkVariant, String> {
    fn me_l(self, error_payload: impl Into<String>)
    -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| {
            let next_link = Box::new(ErrorLink_(e, 
                NextLink::None(Backtrace::capture()))
            );
            ErrorLink_(error_payload.into(), NextLink::Some(next_link))
        })
    }

    fn me_al(self) -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(
            e,
            NextLink::None(Backtrace::capture())
        ))
    }
}

#[cfg(feature = "nightly")]
impl<OkVariant, FromPayload: Display> ResultExt<OkVariant, String> 
for Result<OkVariant, ErrorLink_<FromPayload>> {
    default fn me_l(self, error_payload: impl Into<String>)
    -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| {
            let next_link = Box::new(ErrorLink_(e.0.to_string(), e.1));
            ErrorLink_(
                error_payload.into(),
                NextLink::Some(next_link)
            )
        })
    }

    default fn me_al(self) -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(e.0.to_string(), e.1))
    }
}

#[cfg(feature = "nightly")]
impl<OkVariant> ResultExt<OkVariant, String> 
for Result<OkVariant, ErrorLink_<String>> {
    fn me_l(self, error_payload: impl Into<String>)
    -> Result<OkVariant, ErrorLink_<String>> {
        self.map_err(|e| ErrorLink_(
            error_payload.into(),
            NextLink::Some(Box::new(e))
        ))
    }

    fn me_al(self) -> Result<OkVariant, ErrorLink_<String>> {
        self
    }
}
