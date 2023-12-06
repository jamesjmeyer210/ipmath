use std::fmt::{Display, Formatter};
use std::net::AddrParseError;
use std::str::FromStr;
use crate::net::{Ipv4Address, Ipv6Address};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum IpAddress {
    V4(Ipv4Address),
    V6(Ipv6Address)
}

impl FromStr for IpAddress {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ipv4 = Ipv4Address::from_str(s).map(|ipv4|IpAddress::V4(ipv4));
        if ipv4.is_ok() {
            ipv4
        }
        else {
            Ipv6Address::from_str(s).map(|ipv6|IpAddress::V6(ipv6))
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IpAddress::V4(v4) => Ipv4Address::fmt(v4, f),
            IpAddress::V6(v6) => Ipv6Address::fmt(v6, f)
        }
    }
}

impl IpAddress {
    pub fn is_ipv4(&self) -> bool {
        match self {
            IpAddress::V4(_) => true,
            IpAddress::V6(_) => false
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match self {
            IpAddress::V4(_) => false,
            IpAddress::V6(_) => true
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::net::ip_address::IpAddress;

    #[test]
    fn from_str_returns_ipv4() {
        let ip = IpAddress::from_str("127.0.0.1");
        assert!(ip.is_ok());

        let ip = ip.unwrap();
        assert!(ip.is_ipv4())
    }
}