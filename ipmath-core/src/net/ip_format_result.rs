use std::fmt::{Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::convert::ConversionResult;
use crate::err::IpParseError;
use crate::net::{IpAddress, IpFormat, Ipv4Address};

#[derive(Debug)]
pub(crate) enum IpFormatResult {
    Ipv4Int(u32),
    Ipv4Default(Ipv4Addr),
    Ipv6Int(u128),
    Ipv6Default(Ipv6Addr)
}

impl Display for IpFormatResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ipv4Default(x) => Ipv4Addr::fmt(x, f),
            Self::Ipv4Int(x) => u32::fmt(x, f),
            Self::Ipv6Default(x) => Ipv6Addr::fmt(x, f),
            Self::Ipv6Int(x) => u128::fmt(x, f)
        }
    }
}

/*impl Into<ConversionResult> for IpFormatResult {
    fn into(self) -> ConversionResult {
        ConversionResult::from(self)
    }
}*/

impl <'a>TryFrom<&'a str> for IpFormatResult {
    type Error = IpParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let attempts: Vec<fn(&str) -> Result<IpFormatResult,IpParseError<'a>>> = vec![
            IpFormatResult::try_as_ipv4_default,
            IpFormatResult::try_as_ipv6_default,
            IpFormatResult::try_as_ipv4_int,
            IpFormatResult::try_as_ipv6_int
        ];

        attempts.iter().map(|f|f(value))
            .filter_map(|x|x.ok())
            .next()
            .ok_or(IpParseError::ParseStrError(value))
    }
}

impl IpFormatResult {
    pub(crate) fn get_ip_format(&self) -> IpFormat {
        match self {
            IpFormatResult::Ipv4Default(_) => IpFormat::Ipv4Default,
            IpFormatResult::Ipv4Int(_) => IpFormat::Ipv4Int,
            IpFormatResult::Ipv6Default(_) => IpFormat::Ipv6Default,
            IpFormatResult::Ipv6Int(_) => IpFormat::Ipv6Int,
        }
    }

    /// Attempts to convert an `IpFormatResult` to an `IpFormatResult` of a different variant based
    /// on the `target` `IpFormat`.
    pub(crate) fn try_convert(self, target: IpFormat) -> Result<IpFormatResult,IpParseError<'static>> {
        if self.get_ip_format() == target {
            return Ok(self);
        }

        match target {
            IpFormat::Ipv4Int=> {
                let ip = IpAddress::from(self);
                let ipv4 = ip.unwrap_v4_unchecked();
                Ok(IpFormatResult::Ipv4Int(ipv4.as_u32()))
            },
            IpFormat::Ipv4Default => {
                let ip = IpAddress::from(self);
                let ipv4 = ip.unwrap_v4_unchecked();
                Ok(IpFormatResult::Ipv4Default(ipv4.into()))
            }
            IpFormat::Ipv6Int => {
                let ip = IpAddress::from(self);
                let ipv6 = ip.unwrap_v6_unchecked();
                Ok(IpFormatResult::Ipv6Int(ipv6.as_u128()))
            },
            IpFormat::Ipv6Default => {
                let ip = IpAddress::from(self);
                let ipv6 = ip.unwrap_v6_unchecked();
                Ok(IpFormatResult::Ipv6Default(ipv6.into()))
            },
        }
    }

    /// Tries to convert a `&str` to an `IpFormatResult` based on the provided `IpFormat`
    pub(crate) fn try_from_format(f: IpFormat, s: &str) -> Result<IpFormatResult,IpParseError> {
        match f {
            IpFormat::Ipv4Default => IpFormatResult::try_as_ipv4_default(s),
            IpFormat::Ipv4Int => IpFormatResult::try_as_ipv4_int(s),
            IpFormat::Ipv6Default => IpFormatResult::try_as_ipv6_default(s),
            IpFormat::Ipv6Int => IpFormatResult::try_as_ipv6_int(s)
        }
    }

    fn try_as_ipv4_int<'a>(s: &str) -> Result<IpFormatResult, IpParseError<'a>> {
        s.parse::<u32>()
            .map(|x|IpFormatResult::Ipv4Int(x))
            .map_err(|e|IpParseError::from(e))
    }

    fn try_as_ipv4_default<'a>(s: &str) -> Result<IpFormatResult,IpParseError<'a>> {
        s.parse::<Ipv4Addr>()
            .map(|x|IpFormatResult::Ipv4Default(x))
            .map_err(|e|IpParseError::from(e))
    }

    fn try_as_ipv6_int<'a>(s: &str) -> Result<IpFormatResult,IpParseError<'a>> {
        s.parse::<u128>()
            .map(|x|IpFormatResult::Ipv6Int(x))
            .map_err(|e|IpParseError::from(e))
    }

    fn try_as_ipv6_default<'a>(s: &str) -> Result<IpFormatResult,IpParseError<'a>> {
        s.parse::<Ipv6Addr>()
            .map(|x|IpFormatResult::Ipv6Default(x))
            .map_err(|e|IpParseError::from(e))
    }
}

#[cfg(test)]
mod test {
    use super::IpFormatResult;

    #[test]
    fn try_from_returns_ok(){
        let x = IpFormatResult::try_from("9999");
        assert!(x.is_ok());

        let x = IpFormatResult::try_from("198.0.0.1");
        assert!(x.is_ok());

        let x = IpFormatResult::try_from("5294967295");
        assert!(x.is_ok());

        let x = IpFormatResult::try_from("2345:0425:2CA1:0000:0000:0567:5673:23b5");
        assert!(x.is_ok());
    }

    #[test]
    fn try_from_returns_err(){
        let x = IpFormatResult::try_from("501.999.0.2");
        assert!(x.is_err());
    }
}