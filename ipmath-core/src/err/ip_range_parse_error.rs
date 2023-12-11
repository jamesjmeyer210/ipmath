//! Errors that can occur when parsing an `IpRangeFormat`.
use crate::err::{CountryCodeParseError, IpParseError, StockCodeParseError};

pub enum IpRangeParseError<'a> {
    /// An expected `char` delimiter is missing
    MissingDelimiter(char),
    IpParseError(IpParseError<'a>),
    InvalidCountryCode(CountryCodeParseError<'a>),
    InvalidStockCode(StockCodeParseError<'a>)
}