use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::AddrParseError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum IpParseError<'a>
{
    ParseIntError(ParseIntError),
    AddrParseError(AddrParseError),
    ParseStrError(&'a str),
}

impl Display for IpParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IpParseError::AddrParseError(e) => AddrParseError::fmt(e, f),
            IpParseError::ParseIntError(e) => ParseIntError::fmt(e, f),
            IpParseError::ParseStrError(e) => str::fmt(e, f)
        }
    }
}

impl Error for IpParseError<'_> {

}

impl From<ParseIntError> for IpParseError<'_> {
    fn from(value: ParseIntError) -> Self {
        IpParseError::ParseIntError(value)
    }
}

impl From<AddrParseError> for IpParseError<'_> {
    fn from(value: AddrParseError) -> Self {
        IpParseError::AddrParseError(value)
    }
}