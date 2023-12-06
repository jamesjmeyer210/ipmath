use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, Ipv6Addr};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Ipv6Address {
    _inner: Ipv6Addr
}

impl FromStr for Ipv6Address {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{
            _inner: Ipv6Addr::from_str(s)?
        })
    }
}

impl Display for Ipv6Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ipv6Addr::fmt(&self._inner, f)
    }
}