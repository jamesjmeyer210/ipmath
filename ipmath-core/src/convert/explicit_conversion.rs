use crate::net::IpFormat;

pub struct ExplicitConversion<'a> {
    pub(crate) f_in: IpFormat,
    pub(crate) f_out: IpFormat,
    pub(crate) ip_addr: &'a str,
}

impl <'a> ExplicitConversion<'a> {
    pub fn new(f_in: IpFormat, f_out: IpFormat, ip_addr: &'a str) -> Self {
        Self {
            f_in,
            f_out,
            ip_addr,
        }
    }
}