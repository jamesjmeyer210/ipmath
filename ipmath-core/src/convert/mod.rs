mod explicit_conversion;
mod implicit_conversion;
mod conversion;

pub type ExplicitConversion<'a> = explicit_conversion::ExplicitConversion<'a>;
pub type ImplicitConversion<'a> = implicit_conversion::ImplicitConversion<'a>;