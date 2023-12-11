mod ip_parse_error;
mod ip_conversion_error;
mod country_code_parse_error;
mod stock_code_parse_error;
mod ip_range_parse_error;

pub type IpParseError<'a> = ip_parse_error::IpParseError<'a>;
pub type IpConversionError<'a> = ip_conversion_error::IpConversionError<'a>;
pub type CountryCodeParseError<'a> = country_code_parse_error::CountryCodeParseError<'a>;
pub type StockCodeParseError<'a> = stock_code_parse_error::StockCodeParseError<'a>;
pub type IpRangeParseError<'a> = ip_range_parse_error::IpRangeParseError<'a>;