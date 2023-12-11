pub(crate) struct RangeName<'a> {
    _inner: &'a str
}

impl <'a>From<&'a str> for RangeName<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            _inner: value
        }
    }
}