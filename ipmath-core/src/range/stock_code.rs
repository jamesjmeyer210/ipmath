use crate::err::StockCodeParseError;

pub(crate) struct StockCode<'a> {
    _inner: &'a str,
}

impl <'a>TryFrom<&'a str> for StockCode<'a> {
    type Error = StockCodeParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.len() {
            2 => Ok(Self {
                _inner: value,
            }),
            _ => Err(StockCodeParseError::from(value))
        }
    }
}