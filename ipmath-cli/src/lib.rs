use clap::ValueEnum;
use ipmath_core::net::IpFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Ipv4Int,
    Ipv4Default,
    Ipv6Int,
    Ipv6Default,
}

impl Into<IpFormat> for Format {
    fn into(self) -> IpFormat {
        match self {
            Format::Ipv4Int => IpFormat::Ipv4Int,
            Format::Ipv4Default => IpFormat::Ipv4Default,
            Format::Ipv6Int => IpFormat::Ipv6Int,
            Format::Ipv6Default => IpFormat::Ipv6Default,
        }
    }
}