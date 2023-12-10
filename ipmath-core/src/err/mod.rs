mod ip_parse_error;
mod ip_conversion_error;

pub type IpParseError<'a> = ip_parse_error::IpParseError<'a>;
pub type IpConversionError<'a> = ip_conversion_error::IpConversionError<'a>;