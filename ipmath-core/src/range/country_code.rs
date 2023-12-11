use crate::err::CountryCodeParseError;

pub(crate) struct CountryCode<'a> {
    _inner: &'a str
}

impl <'a>TryFrom<&'a str> for CountryCode<'a> {
    type Error = CountryCodeParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.len() {
            2 => Ok(Self {
                _inner: value,
            }),
            _ => Err(CountryCodeParseError::from(value))
        }
    }
}