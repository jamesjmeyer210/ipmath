use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::err::IpFormatParseError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IpFormat {
    Ipv4Int,
    Ipv4Default,
    Ipv6Int,
    Ipv6Default,
}

impl FromStr for IpFormat {
    type Err = IpFormatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::key_values()
            .get_key_value(s)
            .map(|kvp|*kvp.1)
            .ok_or(IpFormatParseError::default())
    }
}

impl IpFormat {
    pub fn str_opts() -> Vec<&'static str> {
        Self::key_values().drain().map(|kvp|kvp.0).collect()
    }

    pub(crate) fn opposite(&self) -> IpFormat {
        match self {
            IpFormat::Ipv4Default => IpFormat::Ipv4Int,
            IpFormat::Ipv4Int => IpFormat::Ipv4Default,
            IpFormat::Ipv6Default => IpFormat::Ipv6Int,
            IpFormat::Ipv6Int => IpFormat::Ipv6Default,
        }
    }

    fn key_values() -> HashMap<&'static str, IpFormat> {
        HashMap::from([
            (stringify!(Ipv4Int), IpFormat::Ipv4Int),
            (stringify!(Ipv4Default), IpFormat::Ipv4Default),
            (stringify!(Ipv6Int), IpFormat::Ipv6Int),
            (stringify!(Ipv6Default), IpFormat::Ipv6Default),
        ])
    }
}

impl Display for IpFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}