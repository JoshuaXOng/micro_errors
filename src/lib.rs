#![cfg_attr(feature = "nightly", feature(min_specialization))]

mod error_chain;
mod error_link_;
mod linkable_results;
#[cfg(feature = "nightly")]
mod result_ext;

use std::{any::Any, backtrace::Backtrace, fmt::Display};

#[allow(deprecated)]
pub use error_chain::{ErrorLink, ErrorChain};
pub use linkable_results::{LinkableResult1of2, LinkableResult2of2};
#[cfg(feature = "nightly")]
pub use result_ext::ResultExt;
pub use error_link_::{ErrorLink_, NextLink, ErrorLinkable};

#[cfg(test)]
mod tests {
    #[allow(deprecated)]
    use crate::ErrorChain;
    use crate::ErrorLink_;
    use crate::ErrorLinkable;
    use crate::NextLink;
    #[cfg(feature = "nightly")]
    use crate::ResultExt;
    use std::backtrace::Backtrace;
    use crate::LinkableResult1of2;
    use crate::LinkableResult2of2;

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
            'B'
        }
    }

    #[test]
    fn test_favour_concrete() {
        assert!(B::letter() == 'B');
    }

    fn is_output_default(default_output: &str) {
        assert_eq!(
            default_output
                .matches("Link no. 0: Higher level error.\nLink no. 1: Underlying error.")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(default_output);
    }
    fn has_only_one_backtrace(formatted_link: &str) {
        assert_eq!(
            formatted_link 
                .matches("Approximate backtrace of link no. ")
                .collect::<Vec<_>>()
                .len(),
            1
        );
    }

    #[test]
    #[allow(non_snake_case)]
    #[allow(deprecated)]
    fn test__chaining_non_error_chain() {
        let format_output = format!(
            "{}", 
            Err::<(), _>(std::io::Error::other("Underlying error."))
                .map_err(ErrorChain::link_fn("Higher level error."))
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_link_() {
        let error_link: ErrorLink_<String> = Err::<(), _>(std::io::Error::other("Underlying error."))
            .map_err(|e| e.as_link())
            .map_err(|e| e.link("Higher level error."))
            .expect_err("look above");
        let format_output = format!(
            "{}", error_link
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[test]
    #[allow(deprecated)]
    #[allow(non_snake_case)]
    fn test__chaining_error_chain() {
        let format_output = format!(
            "{}", 
            Err::<(), _>(ErrorChain::start("Underlying error."))
                .map_err(ErrorChain::link_fn("Higher level error."))
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_error_link_() {
        let error_link: ErrorLink_<String> = Err::<(), _>(ErrorLink_::new_string("Underlying error."))
            .map_err(|e| e.as_link() as ErrorLink_<String>)
            .map_err(|e| e.link("Higher level error."))
            .expect_err("look above");
        let format_output = format!("{error_link}");
        println!("{}", format_output);
        is_output_default(&format_output);
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
        let error_link: ErrorLink_<String> = Err::<(), _>(ErrorLink_::new_i32(100))
            .map_err(|e| e.link("Higher level error."))
            .expect_err("look above");
        let mut format_output = format!("{error_link}");
        println!("{}", format_output);
        assert_eq!(
            format_output 
                .matches("Link no. 0: Higher level error.\nLink no. 1: 100")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(&format_output);

        let error_link: ErrorLink_<String> = Err::<(), _>(ErrorLink_::new_reason(ErrorReasons::One))
            .map_err(|e| e.link("Higher level error."))
            .expect_err("look above");
        format_output = format!("{}", error_link);
        println!("{}", format_output);
        assert_eq!(
            format_output 
                .matches("Link no. 0: Higher level error.\nLink no. 1: First reason for underlying error.")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(&format_output);

        match Err::<(), _>(ErrorLink_::new_reason(ErrorReasons::Two)) {
            Err(error_chain) if error_chain.0 == ErrorReasons::Two => {
                format_output = format!("{}", error_chain);
                println!("{}", format_output);
                assert_eq!(
                    format_output 
                        .matches("Link no. 0: Second reason for underlying error.")
                        .collect::<Vec<_>>()
                        .len(),
                    1
                );
                has_only_one_backtrace(&format_output);       
            },
            _ => panic!("look above"),
        }
    }

    #[test]
    #[allow(deprecated)]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_chain_and_non_error_trait() {
        let format_output = format!(
            "{}", 
            Err::<(), _>(String::from("Underlying error."))
                .map_err(ErrorChain::onboard_fn("Higher level error."))
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__chaining_non_error_link_and_non_error_trait() {
        let format_output = format!(
            "{}", 
            Err::<(), _>(String::from("Underlying error."))
                .map_err(|e| e.link("Higher level error."))
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__resultext__chaining_non() {
        let format_output = format!(
            "{}", 
            Err::<(), _>(String::from("Underlying error."))
                .map_err(|e| e.link("Higher level error."))
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[cfg(feature = "nightly")]
    #[test]
    #[allow(non_snake_case)]
    fn test__resultext__chaining_error_link__string_payload() {
        let format_output = format!(
            "{}",
            Err::<(), _>(ErrorLink_::new_string("Underlying error."))
                .me_al()
                .me_l("Higher level error.")
                .expect_err("look above")
        );
        println!("{}", format_output);
        is_output_default(&format_output);
    }

    #[cfg(feature = "nightly")]
    #[test]
    #[allow(non_snake_case)]
    fn test__resultext__chaining_error_link__non_string_payload() {
        let format_output = format!(
            "{}",
            Err::<(), _>(ErrorLink_::new_i32(100))
                .me_al()
                .expect_err("look above")
        );
        println!("{}", format_output);
        assert_eq!(
            format_output 
                .matches("100")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(&format_output);
    }

    #[cfg(feature = "nightly")]
    #[test]
    #[allow(non_snake_case)]
    fn test__resultext__chaining_non_error_link__string_payload() {
        let format_output = format!(
            "{}",
            Err::<(), _>(String::from("Underlying error."))
                .me_al()
                .expect_err("look above")
        );
        println!("{}", format_output);
        assert_eq!(
            format_output 
                .matches("Underlying error.")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(&format_output);
    }

    #[cfg(feature = "nightly")]
    #[test]
    #[allow(non_snake_case)]
    fn test__resultext__chaining_non_error_link__non_string_payload() {
        let format_output = format!(
            "{}",
            Err::<(), _>(100)
                .me_al()
                .expect_err("look above")
        );
        println!("{}", format_output);
        assert_eq!(
            format_output 
                .matches("100")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        has_only_one_backtrace(&format_output);
    }

    #[allow(dead_code)]
    struct DisplayableAndIntoStringable(String);
    impl From<DisplayableAndIntoStringable> for String {
        fn from(value: DisplayableAndIntoStringable) -> Self {
            value.0
        }
    }
    impl std::fmt::Display for DisplayableAndIntoStringable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "DisplayableAndIntoStringable")
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__link_conversion__to_error_link_string() {
        let _ = || -> Result<(), ErrorLink_<String>> {
            Err::<(), _>(ErrorLink_::new("These `Result`s would be typically from function calls."))?;
            Err::<(), _>(String::from(
                "So, if the error is an `ErrorLink_` of same payload, or the payload type itself, \
                `?` can be used."
            ))?;
            #[cfg(feature = "nightly")]
            Err::<(), _>(std::io::Error::other(
                "`ErrorLink_<String>` is like a terminal type. `me_as_slink()?` can always be called, \
                and all can turn into it."
            )).me_as_slink()?;
            Err::<(), _>(ErrorLink_::<DisplayableAndIntoStringable>::new(
                DisplayableAndIntoStringable(String::from(
                    "`impl<T> from <T> for T` exists, so `?` cannot cover all `ErrorLink_` to \
                    `ErrorLink_`s. Hence `me_as_link`."
                ))
            )).me_as_link()?;
            Ok(())
        }();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__adding_link__to_error_link_string() {
        let _ = || -> Result<(), ErrorLink_<String>> {
            Err::<(), _>(ErrorLink_::<std::io::Error>::new(
                std::io::Error::other("Some data relating to an unhappy code path.")
            ))
                .me_link("Some more information.")?;
            Ok(())
        }();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__replacing_payload__to_error_link_string() {
        let _ = || -> Result<(), ErrorLink_<String>> {
            Err::<(), _>(ErrorLink_::<std::io::Error>::new(std::io::Error::other("Something ugly.")))
                .map_err(|_| ErrorLink_::new("Get replaced."))?;
            Err::<(), _>(ErrorLink_::<std::io::Error>::new(std::io::Error::other("Something ugly.")))
                .map_err(|e| e.replace("Get replaced. But keep its linkage."))?;
            Ok(())
        }();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__link_conversion__to_error_link_non_string() {
        let _ = || -> Result<(), ErrorLink_<i32>> {
            Err::<(), _>(ErrorLink_::new(88))?;
            Err::<(), _>(44)?;
            Err::<(), _>(ErrorLink_::<i8>::new(8)).me_as_link()?;
            Ok(())
        }();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__adding_link__to_error_link_non_string() {
        let _ = || -> Result<(), ErrorLink_<i32>> {
            Err::<(), _>(ErrorLink_::new_string(""))
                .me_link(100)?;
            Ok(())
        }();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test__replacing_payload__to_error_link_non_string() {
        let _ = || -> Result<(), ErrorLink_<i32>> {
            Err::<(), _>(ErrorLink_::<std::io::Error>::new(std::io::Error::other("Something ugly.")))
                .map_err(|_| ErrorLink_::new(3))?;
            Err::<(), _>(ErrorLink_::<std::io::Error>::new(std::io::Error::other("Something ugly.")))
                .map_err(|e| e.replace(1))?;
            Ok(())
        }();
    }
}

// TODO: struct String_(&'static str);

trait DisplayableAny: Display + Any {}
impl<T: Display + Any> DisplayableAny for T {}

enum NL {
    None(Backtrace),
    Some(Box<EL<Box<dyn DisplayableAny>>>)
}

struct EL<Payload: DisplayableAny>(Payload, NL);

impl<Payload: DisplayableAny> EL<Payload> {
    fn new(link_payload: impl Into<Payload>) -> Self {
        Self(link_payload.into(), NL::None(Backtrace::capture()))
    }
}

impl<T: DisplayableAny> From<T> for EL<T> {
    fn from(value: T) -> Self {
        Self(value, NL::None(Backtrace::capture()))
    }
}

impl<T: DisplayableAny> From<T> for EL<Box<T>> {
    fn from(value: T) -> Self {
        Self(Box::new(value), NL::None(Backtrace::capture()))
    }
}

trait RE<O, E, FP: DisplayableAny> {
    fn add_link<TP: DisplayableAny>(self, link_payload: impl Into<TP>) -> Result<O, EL<TP>>;
    fn as_anylink(self) -> Result<O, EL<Box<dyn DisplayableAny>>>;
    fn convert_link<TP: From<FP> + DisplayableAny>(self) -> Result<O, EL<TP>>;
}

impl<O, E: DisplayableAny> RE<O, E, E> for Result<O, E> {
    fn add_link<TP: DisplayableAny>(self, link_payload: impl Into<TP>) -> Result<O, EL<TP>> {
        self.map_err(|e| {
            let next_link = Box::new(EL(
                Box::new(e) as Box<dyn DisplayableAny>, 
                NL::None(Backtrace::capture())
            ));
            EL(link_payload.into(), NL::Some(next_link))
        })
    }

    fn as_anylink(self) -> Result<O, EL<Box<dyn DisplayableAny>>> {
        self.map_err(|e| EL::new(Box::new(e) as Box<dyn DisplayableAny>))
    }

    fn convert_link<TP: From<E> + DisplayableAny>(self) -> Result<O, EL<TP>> {
        self.map_err(|e| EL::new(e))
    }
}

impl<O, E: DisplayableAny> RE<O, EL<E>, E> for Result<O, EL<E>> {
    fn add_link<TP: DisplayableAny>(self, link_payload: impl Into<TP>) -> Result<O, EL<TP>> {
        todo!()
    }

    fn as_anylink(self) -> Result<O, EL<Box<dyn DisplayableAny>>> {
        todo!()
    }

    fn convert_link<TP: From<E> + DisplayableAny>(self) -> Result<O, EL<TP>> {
        todo!()
    }
}

impl<O, E: DisplayableAny> RE<O, EL<E>, E> for Result<O, EL<Box<E>>> {
    fn add_link<TP: DisplayableAny>(self, link_payload: impl Into<TP>) -> Result<O, EL<TP>> {
        todo!()
    }

    fn as_anylink(self) -> Result<O, EL<Box<dyn DisplayableAny>>> {
        todo!()
    }

    fn convert_link<TP: From<E> + DisplayableAny>(self) -> Result<O, EL<TP>> {
        todo!()
    }
}

fn contract() -> Result<(), EL<Box<dyn DisplayableAny>>> {
    Err::<(), _>(String::new()).add_link::<String>("").as_anylink()?;
    Err::<(), _>(Box::new(String::new())).add_link::<String>("").as_anylink()?;
    Err::<(), _>(EL::<String>::new(String::new())).add_link::<String>("").as_anylink()?;
    Err::<(), _>(EL::<Box<String>>::new(Box::new(String::new()))).add_link::<String>("").as_anylink()?;

    Err::<(), _>(String::new()).add_link::<String>("").as_anylink()?;

    Ok(())
}

enum Reason {
    One
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<String> for Reason {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<Box<String>> for Reason {
    fn from(value: Box<String>) -> Self {
        todo!()
    }
}

fn expand() -> Result<(), EL<Reason>> {
    Err::<(), _>(String::new()).add_link(Reason::One)?;
    Err::<(), _>(Box::new(String::new())).add_link(Reason::One)?;
    Err::<(), _>(EL::<String>::new(String::new())).add_link(Reason::One)?;
    Err::<(), _>(EL::<Box<String>>::new(Box::new(String::new()))).add_link(Reason::One)?;

    Err::<(), _>(String::new()).convert_link()?;
    Err::<(), _>(Box::new(String::new())).convert_link()?;
    Err::<(), _>(EL::<String>::new(String::new())).convert_link()?;
    Err::<(), _>(EL::<Box<String>>::new(Box::new(String::new()))).convert_link()?;

    contract().add_link(Reason::One)?;

    Err::<(), _>(Reason::One)?;
    Err::<(), _>(EL::new(Reason::One))?;

    Ok(())
}
