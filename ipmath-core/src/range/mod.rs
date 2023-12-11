mod ip_range_result;
mod ip_range_format;
mod country_code;
mod stock_code;
mod range_name;

type CountryCode<'a> = country_code::CountryCode<'a>;
type StockCode<'a> = stock_code::StockCode<'a>;
type RangeName<'a> = range_name::RangeName<'a>;