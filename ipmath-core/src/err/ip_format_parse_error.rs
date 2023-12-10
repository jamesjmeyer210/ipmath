use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct IpFormatParseError;

impl Default for IpFormatParseError {
    fn default() -> Self {
        Self
    }
}

impl Display for IpFormatParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for IpFormatParseError {

}