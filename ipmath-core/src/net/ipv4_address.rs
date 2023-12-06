use std::fmt::Display;
use std::net::{Ipv4Addr};

pub struct Ipv4Address {
    _inner: Ipv4Addr
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octets = self.octets();
        write!(f, "{}", format!("{}.{}.{}.{}", octets[0], octets[0], octets[0], octets[0]))
    }
}

impl Ipv4Address {
    fn octets(&self) -> [u8;4] {
        self._inner.octets()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
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
}