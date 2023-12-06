mod ipv4_address;
mod ipv6_address;
mod ip_address;
mod ip_encoding;

pub type Ipv4Address = ipv4_address::Ipv4Address;
pub type Ipv6Address = ipv6_address::Ipv6Address;
pub type IpAddress = ip_address::IpAddress;
pub type IpEncoding = ip_encoding::IpEncoding;