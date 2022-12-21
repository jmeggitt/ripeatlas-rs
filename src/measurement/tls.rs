use crate::general::AddressFamily;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Tls<'a> {
    /// address family, 4 or 6 (integer)
    pub af: Option<AddressFamily>,
    /// [optional] error sent by server (see RFC 5246, Section 7.2)
    pub alert: Option<Alert>,
    /// [optional] results of query, not present if "alert" is present (array of strings)
    /// Each element of the array is a string containing a base 64 encoded certificate.
    /// Newlines are replaced with "\n"
    pub cert: Option<Vec<Cow<'a, str>>>,
    /// IP address of the destination (string)
    pub dst_addr: Option<Cow<'a, str>>,
    /// name of the destination (string)
    pub dst_name: Cow<'a, str>,
    /// port name (string)
    pub dst_port: Cow<'a, str>,
    /// "SSL" or "TLS" (string)
    pub method: Option<Method>,
    /// [optional] response time in milli seconds from starting to connect to receiving the
    /// certificates (float)
    pub rt: Option<f64>,
    /// [optional] cipher selected by server as a hexadecimal number (string)
    pub server_cipher: Option<Cow<'a, str>>,
    /// [optional] time in milli seconds that it took to connect (over TCP) to the target (float)
    pub ttc: Option<f64>,
    /// (SSL/TLS) protocol version (string)
    pub ver: Option<Cow<'a, str>>,
    /// description of error (string)
    pub err: Option<Cow<'a, str>>,

    /// description of error (string)
    pub error: Option<Cow<'a, str>>,
    /// description of error (string)
    pub dnserr: Option<Cow<'a, str>>,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Alert {
    /// AlertLevel (integer)
    level: i64,
    /// AlertDescription (integer)
    description: i64,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Method {
    SSL,
    TLS,
}
