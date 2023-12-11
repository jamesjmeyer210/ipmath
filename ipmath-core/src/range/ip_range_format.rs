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