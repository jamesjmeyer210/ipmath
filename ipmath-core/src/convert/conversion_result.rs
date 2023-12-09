use std::fmt::{Display, Formatter};
use crate::net::{IpFormatResult};

pub struct ConversionResult {
    _ip_format: IpFormatResult,
}

impl Display for ConversionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        IpFormatResult::fmt(&self._ip_format, f)
    }
}

impl From<IpFormatResult> for ConversionResult {
    fn from(value: IpFormatResult) -> Self {
        Self {
            _ip_format: value
        }
    }
}