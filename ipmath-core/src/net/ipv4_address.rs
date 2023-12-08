use std::fmt::{Display, Formatter, write};
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Ipv4Address {
    _inner: Ipv4Addr
}

impl From<Ipv4Addr> for Ipv4Address {
    fn from(value: Ipv4Addr) -> Self {
        Self {
            _inner: value
        }
    }
}

impl FromStr for Ipv4Address {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            _inner: Ipv4Addr::from_str(s)?
        })
    }
}

impl From<u32> for Ipv4Address {
    fn from(value: u32) -> Self {
        let x1= (value >> 24) as u8;
        let x2= ((value << 8) >> 24) as u8;
        let x3= ((value << 16) >> 24) as u8;
        let x4= ((value << 24) >> 24) as u8;
        Self {
            _inner: Ipv4Addr::new(x1, x2, x3, x4)
        }
    }
}

impl Display for Ipv4Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ipv4Addr::fmt(&self._inner, f)
    }
}

impl Ipv4Address {
    fn octets(&self) -> [u8;4] {
        self._inner.octets()
    }

    fn as_u32(&self) -> u32 {
        let octets = self.octets().map(|x|x as u32);
        (octets[0] << 24) + (octets[1] << 16) + (octets[2] << 8) + octets[3]
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::str::FromStr;
    use super::Ipv4Address;

    #[test]
    fn from_u32_test(){
        let map: HashMap<u32,[u8;4]> = HashMap::from([
            (0, [0, 0, 0, 0]),
            (1, [0, 0, 0, 1]),
            (255, [0, 0, 0, 255]),
            (255, [0, 0, 0, 255]),
            (1<<8, [0, 0, 1, 0]),
            (1<<16, [0, 1, 0, 0]),
            (1<<24, [1, 0, 0, 0]),
            (4294967295, [255, 255, 255, 255])
        ]);

        for i in map {
            let ipv4 = Ipv4Address::from(i.0);
            assert_eq!(ipv4.octets(), i.1);
        }
    }

    #[test]
    fn from_str_test(){
        let s = "127.0.0.1";

        let ipv4 = Ipv4Address::from_str(s);
        assert!(ipv4.is_ok());

        let ipv4 = ipv4.unwrap();
        let x = ipv4.to_string();
        assert_eq!(s, &x);
    }

    #[test]
    fn as_u32_test(){
        let map = HashMap::from([
            (0, "0.0.0.0"),
            (1, "0.0.0.1"),
            (255, "0.0.0.255"),
            (1<<8, "0.0.1.0"),
            (1<<16, "0.1.0.0"),
            (1<<24, "1.0.0.0"),
            (4294967295, "255.255.255.255"),
        ]);

        for kvp in map {
            let ipv4 = Ipv4Address::from(kvp.0);
            let x = ipv4.as_u32();
            assert_eq!(kvp.0, x);

            let s = ipv4.to_string();
            assert_eq!(kvp.1, s.as_str());
        }
    }
}