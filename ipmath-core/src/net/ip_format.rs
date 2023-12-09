use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IpFormat {
    Ipv4Int,
    Ipv4Default,
    Ipv6Int,
    Ipv6Default,
}

impl IpFormat {
    pub(crate) fn opposite(&self) -> IpFormat {
        match self {
            IpFormat::Ipv4Default => IpFormat::Ipv4Int,
            IpFormat::Ipv4Int => IpFormat::Ipv4Default,
            IpFormat::Ipv6Default => IpFormat::Ipv6Int,
            IpFormat::Ipv6Int => IpFormat::Ipv6Default,
        }
    }
}

impl Display for IpFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, stringify!(self))
    }
}