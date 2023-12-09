mod explicit_conversion;
mod implicit_conversion;
mod conversion;
mod conversion_result;

pub type ExplicitConversion<'a> = explicit_conversion::ExplicitConversion<'a>;
pub type ImplicitConversion<'a> = implicit_conversion::ImplicitConversion<'a>;
pub type ConversionResult = conversion_result::ConversionResult;