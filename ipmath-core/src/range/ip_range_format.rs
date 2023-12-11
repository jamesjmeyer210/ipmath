use std::str::FromStr;
use crate::err::{IpParseError, IpRangeParseError};
use crate::net::{Ipv4Address, Ipv6Address};
use crate::range::{CountryCode, RangeName, StockCode};

pub(crate) enum IpRangeFormat<'a> {
    Ipv4Cidr(Ipv4Address, u8),
    Ipv4Delim(Ipv4Address, Ipv4Address),
    Ipv4IntDelim(u32, u32),
    Ipv6Cidr(Ipv6Address, u8),
    Ipv6Delim(Ipv6Address, Ipv6Address),
    Ipv6IntDelim(u128, u128),
    CountryCode(CountryCode<'a>),
    StockCode(StockCode<'a>),
    RangeName(RangeName<'a>)
}

impl <'a>IpRangeFormat<'a> {
    fn try_as_ipv4_cidr(s: &'a str) -> Result<IpRangeFormat,IpRangeParseError> {
        let split: Vec<&str> = s.split('/').collect();
        if split.len() != 2 {
            return Err(IpRangeParseError::MissingDelimiter('/'))
        }

        let addr = Ipv4Address::from_str(split[0])
            .map_err(|e|IpParseError::from(e))
            .map_err(|e|IpRangeParseError::IpParseError(e))?;

        let bits = split[1].parse::<u8>()
            .map_err(|e|IpParseError::from(e))
            .map_err(|e|IpRangeParseError::IpParseError(e))?;

        Ok(Self::Ipv4Cidr(addr, bits))
    }
}

#[cfg(test)]
mod test {
    use super::IpRangeFormat;

    #[test]
    fn try_as_ipv4_cidr_returns_ok(){
        let r = IpRangeFormat::try_as_ipv4_cidr("1.0.0.1/32");
        assert!(r.is_ok())
    }

    #[test]
    fn try_as_ipv4_cidr_returns_err(){
        let r = IpRangeFormat::try_as_ipv4_cidr("256.0.0.1/32");
        assert!(r.is_err());

        let r = IpRangeFormat::try_as_ipv4_cidr("1.0.0.1/256");
        assert!(r.is_err());
    }
}