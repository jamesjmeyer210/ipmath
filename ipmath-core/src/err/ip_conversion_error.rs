use std::fmt::{Display, Formatter};
use crate::err::IpParseError;
use crate::net::IpFormat;

pub enum IpConversionError<'a> {
    ParseError(IpParseError<'a>),
    CannotConvert(IpFormat, IpFormat),
}

impl <'a>From<IpParseError<'a>> for IpConversionError<'a> {
    fn from(value: IpParseError<'a>) -> Self {
        Self::ParseError(value)
    }
}

impl Display for IpConversionError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => IpParseError::fmt(e, f),
            Self::CannotConvert(f_in, f_out) => {
                write!(f, "Cannot convert {f_in} to {f_out}")
            }
        }
    }
}