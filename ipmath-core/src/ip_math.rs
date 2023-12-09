use crate::convert::ConversionResult;
use crate::err::{IpConversionError, IpParseError};
use crate::net::{IpAddress, IpFormat, IpFormatResult};

pub struct IpMath;

impl IpMath {
    pub fn convert(ip: &str, format_in: Option<IpFormat>, format_out: Option<IpFormat>) -> Result<ConversionResult,IpConversionError> {
        match (format_in, format_out) {
            (None, None) => Self::convert_implicit_in_out(ip),
            (None, Some(f_out)) => Self::convert_implicit_out(ip, f_out),
            (Some(f_in), None) => Self::convert_implicit_in(ip, f_in),
            (Some(f_in), Some(f_out)) => Self::convert_explicit(ip, f_in, f_out)
        }
    }

    fn convert_implicit_in_out(ip: &str) -> Result<ConversionResult,IpConversionError> {
        let ip_format = IpFormatResult::try_from(ip)?;
        let f_out = ip_format.get_ip_format().opposite();
        todo!()
    }

    fn convert_implicit_in(ip: &str, f_in: IpFormat) -> Result<ConversionResult,IpConversionError> {
        Self::convert_explicit(ip, f_in, f_in.opposite())
    }

    fn convert_implicit_out(ip: &str, f_out: IpFormat) -> Result<ConversionResult,IpConversionError> {
        let ip_format = IpFormatResult::try_from(ip)?;
        todo!()
    }

    fn convert_explicit(ip: &str, f_in: IpFormat, f_out: IpFormat) -> Result<ConversionResult,IpConversionError> {
        if f_in == f_out {
            return IpFormatResult::try_from_format(f_in, ip)
                .map(|x|x.into())
                .map_err(|e|IpConversionError::from(e))
        }

        match (f_in, f_out) {
            (IpFormat::Ipv4Default, IpFormat::Ipv4Int) => todo!(),
            (IpFormat::Ipv4Int, IpFormat::Ipv4Default) => todo!(),
            (IpFormat::Ipv6Default, IpFormat::Ipv6Int) => todo!(),
            (IpFormat::Ipv6Int, IpFormat::Ipv6Default) => todo!(),
            _ => Err(IpConversionError::CannotConvert(f_in, f_out))
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::net::IpFormat;
    use super::IpMath;

    #[test]
    fn convert_explicit_returns_ok_when_formats_match() {
        let ips = HashMap::from([
            ("127.255.101.1", IpFormat::Ipv4Default),
            ("5001", IpFormat::Ipv4Int),
            ("A345:0425:2CA1:0000:FF00:0567:5673:23B5", IpFormat::Ipv6Default),
            ("1020304050607080900000000", IpFormat::Ipv6Int)
        ]);

        for kvp in ips {
            let r = IpMath::convert_explicit(kvp.0, kvp.1, kvp.1);
            assert!(r.is_ok())
        }
    }

    #[test]
    fn convert_explicit_returns_err_for_invalid_conversion() {
        let ips = vec![
            ("127.255.101.1", IpFormat::Ipv4Default, IpFormat::Ipv6Default),
            ("5001", IpFormat::Ipv4Int, IpFormat::Ipv6Int),
            ("A345:0425:2CA1:0000:FF00:0567:5673:23B5", IpFormat::Ipv6Default, IpFormat::Ipv4Int),
            ("1020304050607080900000000", IpFormat::Ipv6Int, IpFormat::Ipv4Default)
        ];

        for i in ips {
            let r = IpMath::convert_explicit(i.0, i.1, i.2);
            assert!(r.is_err());
            let e = r.unwrap_err();
            assert!(e.is_cannot_convert_error())
        }
    }
}