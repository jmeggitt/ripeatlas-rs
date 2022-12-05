use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::measurement::{AddressFamily, Protocol, Response};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Ping<'a> {
    /// address family, 4 or 6 (integer)
    pub af: AddressFamily,
    /// average round-trip time (float)
    pub avg: f64,
    /// IP address of the destination (string)
    ///
    /// > Note: Will not be present in cases where traceroute failed to resolve the host name.
    pub dst_addr: Option<Cow<'a, str>>,
    /// name of the destination (string)
    pub dst_name: Cow<'a, str>,
    /// number of duplicate packets (int)
    pub dup: u32,
    /// maximum round-trip time (float)
    pub max: f64,
    /// minimum round-trip time (float)
    pub min: f64,
    /// "ICMP" (string)
    pub proto: Protocol,
    /// number of packets received (int)
    pub rcvd: u32,
    /// variable content, depending on type of response (array of objects)
    pub result: Vec<Response<'a, PingReply<'a>>>,
    /// number of packets sent (int)
    pub sent: u32,
    /// packet size (data part, not including IP and ICMP header) (int)
    pub size: u64,
    /// time-to-live field in the first reply (missing due to a bug)(int)
    pub ttl: Option<u32>,
    // TODO: Properly handle cases where this field is null
    pub step: Option<i32>,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct PingReply<'a> {
    /// round-trip-time in milliseconds (float)
    pub rtt: f64,
    /// [optional] source address if different from the source address in first reply (string)
    #[serde(rename = "src_Addr")]
    pub src_addr: Option<Cow<'a, str>>,
    /// [optional] time-to-live reply if different from ttl in first reply (int)
    pub ttl: Option<u32>,
    /// [optional] signals that the reply is a duplicate (int)
    pub dup: Option<i32>,
}
