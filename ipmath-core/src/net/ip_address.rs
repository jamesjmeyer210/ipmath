use crate::net::{Ipv4Address, Ipv6Address};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum IpAddress {
    V4(Ipv4Address),
    V6(Ipv6Address)
}