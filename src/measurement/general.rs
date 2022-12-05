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
