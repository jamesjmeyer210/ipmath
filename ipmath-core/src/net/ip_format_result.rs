use std::net::{Ipv4Addr, Ipv6Addr};
use crate::err::IpParseError;

pub(crate) enum IpFormatResult {
    Ipv4Int(u32),
    Ipv4Default(Ipv4Addr),
    Ipv6Int(u128),
    Ipv6Default(Ipv6Addr)
}

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