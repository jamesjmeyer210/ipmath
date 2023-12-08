use crate::convert::{ExplicitConversion, ImplicitConversion};
use crate::err::IpParseError;
use crate::net::{IpFormat, IpFormatResult};

pub(crate) struct Conversion {
    ip_addr: IpFormatResult,
    f_out: IpFormat
}

impl <'a>TryFrom<ImplicitConversion<'a>> for Conversion {
    type Error = IpParseError<'a>;

    fn try_from(value: ImplicitConversion<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            ip_addr: IpFormatResult::try_from(value.ip_addr)?,
            f_out: value.f_out,
        })
    }
}

impl <'a>TryFrom<ExplicitConversion<'a>> for Conversion {
    type Error = IpParseError<'a>;

    fn try_from(value: ExplicitConversion<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            ip_addr: IpFormatResult::try_from(value.ip_addr)?,
            f_out: value.f_out,
        })
    }
}