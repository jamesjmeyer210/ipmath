use crate::net::{IpFormat};

pub struct ImplicitConversion<'a> {
    pub(crate) f_out: IpFormat,
    pub(crate) ip_addr: &'a str,
}