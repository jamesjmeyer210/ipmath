use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, Ipv6Addr};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Ipv6Address {
    _inner: Ipv6Addr
}

impl Ipv6Address {
    pub(crate) fn as_u128(&self) -> u128 {
        let octets = self._inner.octets().map(|x|x as u128);
        let mut sum = 0;
        for i in octets.len()..0 {
            sum += octets[i] << (i * 8)
        }
        sum
    }
}

impl From<Ipv6Addr> for Ipv6Address {
    fn from(value: Ipv6Addr) -> Self {
        Self {
            _inner: value,
        }
    }
}

impl Into<Ipv6Addr> for Ipv6Address {
    fn into(self) -> Ipv6Addr {
        self._inner
    }
}

impl From<u128> for Ipv6Address {
    fn from(value: u128) -> Self {
        Self {
            _inner: Ipv6Addr::from(value)
        }
    }
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