use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::measurement::{AddressFamily, Protocol, Response};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Ntp<'a> {
    /// address family, 4 or 6 (integer)
    pub af: AddressFamily,
    /// name of the destination (string)
    pub dst_name: Cow<'a, str>,
    /// port name (string)
    pub dst_port: Option<Cow<'a, str>>,
    /// leap indicator, values "no", "61", "59", or "unknown" (string)
    pub li: Option<LeapIndicator>,
    /// "server" (string)
    pub mode: Option<Cow<'a, str>>,
    /// poll interval in seconds (float)
    pub poll: Option<f64>,
    /// precision of the server's clock in seconds (float)
    pub precision: Option<f64>,
    /// "UDP" (string)
    pub proto: Protocol,
    /// server's reference clock (string)
    #[serde(rename = "ref-id")]
    pub ref_id: Option<Cow<'a, str>>,
    /// server's reference timestamp in NTP seconds (float)
    #[serde(rename = "ref-ts")]
    pub ref_ts: Option<f64>,
    /// results of query (array of objects)
    pub result: Vec<Response<'a, NtpReply<'a>>>,
    /// round-trip delay from server to stratum 0 time source in seconds (float)
    #[serde(rename = "root-delay")]
    pub root_delay: Option<f64>,
    /// total dispersion to stratum 0 time source in seconds (float)
    #[serde(rename = "root-dispersion")]
    pub root_dispersion: Option<f64>,
    /// distance in hops from server to primary time source (int)
    pub stratum: Option<Stratum>,
    /// NTP protocol version (int)
    pub version: Option<i32>,

    /// IP address of the destination (string)
    pub dst_addr: Option<Cow<'a, str>>,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct NtpReply<'a> {
    /// NTP time the response from the server is received (float)
    #[serde(rename = "final-ts")]
    pub final_ts: f64,
    /// clock offset between client and server in seconds (float)
    pub offset: f64,
    /// NTP time the request was sent (float)
    #[serde(rename = "origin-ts")]
    pub origin_ts: f64,
    /// NTP time the server received the request (float)
    #[serde(rename = "receive-ts")]
    pub receive_ts: f64,
    /// round trip time between client and server in seconds (float)
    pub rtt: f64,
    /// NTP time the server sent the response (float)
    #[serde(rename = "transmit-ts")]
    pub transmit_ts: f64,

    /// total dispersion to stratum 0 time source in seconds (float)
    #[serde(rename = "root-dispersion")]
    pub root_dispersion: Option<f64>,
    /// leap indicator, values "no", "61", "59", or "unknown" (string)
    pub li: Option<LeapIndicator>,
    /// precision of the server's clock in seconds (float)
    pub precision: Option<f64>,
    /// server's reference clock (string)
    #[serde(rename = "ref-id")]
    pub ref_id: Option<Cow<'a, str>>,
    /// server's reference timestamp in NTP seconds (float)
    #[serde(rename = "ref-ts")]
    pub ref_ts: Option<f64>,
    /// distance in hops from server to primary time source (int)
    pub stratum: Option<Stratum>,
    /// round-trip delay from server to stratum 0 time source in seconds (float)
    #[serde(rename = "root-delay")]
    pub root_delay: Option<f64>,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Stratum {
    String(StratumConst),
    Distance(i32),
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum StratumConst {
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum LeapIndicator {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "59")]
    Seconds59,
    #[serde(rename = "61")]
    Seconds61,
    #[serde(rename = "unknown")]
    Unknown,
}
