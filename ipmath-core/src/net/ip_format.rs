use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::num::ParseIntError;
use crate::err::IpParseError;
use crate::net::Ipv6Address;

pub enum IpFormat {
    Ipv4Int,
    Ipv4Default,
    Ipv6Int,
    Ipv6Default,
}

impl IpFormat {
    pub(crate) fn detect(s: &str) -> Option<IpFormat> {
        None
    }
}