use clap::ValueEnum;
use ipmath_core::net::IpEncoding;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Encoding {
    Int,
    Binary,
    Decimal,
    Hex,
    Base64
}

impl Into<IpEncoding> for Encoding {
    fn into(self) -> IpEncoding {
        match self {
            Encoding::Int => IpEncoding::Int,
            Encoding::Binary => IpEncoding::Binary,
            Encoding::Decimal => IpEncoding::Decimal,
            Encoding::Hex => IpEncoding::Hex,
            Encoding::Base64 => IpEncoding::Base64,
        }
    }
}