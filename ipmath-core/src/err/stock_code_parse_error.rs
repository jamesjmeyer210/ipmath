use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct StockCodeParseError<'a> {
    _inner: &'a str
}

impl Display for StockCodeParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a stock code", self._inner)
    }
}

impl Error for StockCodeParseError<'_> {

}