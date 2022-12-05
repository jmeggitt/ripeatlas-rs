use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum AddressFamily {
    IPv4 = 4,
    IPv6 = 6,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Protocol {
    UDP,
    TCP,
    ICMP,
}

#[cfg(not(feature = "chrono"))]
pub type UnixTimestamp = i64;

#[cfg(feature = "chrono")]
pub type UnixTimestamp = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum Response<'a, T> {
    Timeout {
        /// Always "*"
        x: Cow<'a, str>,
    },
    Error {
        /// description of error (string)
        error: Cow<'a, str>,
    },
    DnsError {
        /// DNS resolution failed (string)
        dnserr: Cow<'a, str>,
    },
    Reply(T),
}


