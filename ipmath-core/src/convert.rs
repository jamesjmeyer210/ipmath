use crate::net::IpFormat;

pub struct Explicit<'a> {
    pub(crate) format_in: IpFormat,
    pub(crate) format_out: IpFormat,
    pub(crate) ip_addr: &'a str,
}

impl <'a>Explicit<'a> {
    pub fn new(f_in: IpFormat, f_out: IpFormat, ip_addr: &'a str) -> Self {
        Self {
            format_in: f_in,
            format_out: f_out,
            ip_addr,
        }
    }
}

pub enum ConvertResult {
    Ipv4Int(u32),
    Ipv4Default(String),
    Ipv6Int(u64),
    Ipv6Default(String)
}

impl <'a>TryFrom<Explicit<'a>> for ConvertResult {
    type Error = ();

    fn try_from(value: Explicit) -> Result<Self, Self::Error> {
        match value.format_in {
            IpFormat::Ipv4Default => Err(()),
            IpFormat::Ipv4Int => Err(()),
            IpFormat::Ipv6Default => Err(()),
            IpFormat::Ipv6Int => Err(()),
        }
    }
}