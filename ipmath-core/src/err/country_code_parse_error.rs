use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CountryCodeParseError<'a> {
    _inner: &'a str
}

impl <'a>From<&'a str> for CountryCodeParseError<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            _inner: value
        }
    }
}

impl Display for CountryCodeParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a country code", self._inner)
    }
}

impl Error for CountryCodeParseError<'_> {

}