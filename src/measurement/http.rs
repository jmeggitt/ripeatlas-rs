use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::measurement::{AddressFamily,  UnixTimestamp};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Http<'a> {
    /// request uri (string)
    pub uri: Cow<'a, str>,
    /// results of query (array of objects)
    pub result: Vec<HttpResult<'a>>,
    /// [optional] time to resolve the DNS name (in milli seconds) (float)
    pub ttr: Option<f64>,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum HttpResult<'a> {
    DnsError {
        /// [optional] DNS resolution failed (string)
        dnserr: Cow<'a, str>,
    },
    Reply(HttpReply<'a>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct HttpReply<'a> {
    /// address family, 4 or 6 (integer)
    pub af: AddressFamily,
    /// size of body in octets (int)
    pub bsize: Option<u64>,
    /// target address (string)
    pub dst_addr: Cow<'a, str>,
    /// [optional] other failure (string)
    pub err: Option<Cow<'a, str>>,
    /// [optional] elements are strings. The last string can be empty to indicate the end of enders
    /// or end with "[...]" to indicate truncation (array of strings)
    pub header: Option<Vec<Cow<'a, str>>>,
    /// header size in octets (int)
    pub hsize: Option<u64>,
    /// "GET", "HEAD", or "POST" (string)
    pub method: Method,
    /// [optional] timing results for reply data (array of objects)
    pub readtiming: Option<Vec<ReadTiming>>,
    /// HTTP result code (int)
    pub res: Option<u32>,
    /// time to execute request excluding DNS (float)
    pub rt: Option<f64>,
    /// source address used by probe (string)
    pub src_addr: Option<Cow<'a, str>>,
    /// [optional] sequence number of this result within a group of results, when the 'all' option
    /// is used without the 'combine' option (int)
    pub subid: Option<i64>,
    /// [optional] total number of results within a group (int)
    pub submax: Option<i64>,
    /// [optional] Unix timestamp, when the 'all' option is used with the 'combine' option (int)
    pub time: Option<UnixTimestamp>,
    /// [optional] time to connect to the target (in milli seconds) (float)
    pub ttc: Option<f64>,
    /// [optional] time to first response byte received by measurent code after starting to connect (in milli seconds) (float)
    pub ttfb: Option<f64>,
    /// [optional] time to resolve the DNS name (in milli seconds) (float)
    pub ttr: Option<f64>,
    /// major, minor version of http server (string)
    pub ver: Option<Cow<'a, str>>,
}


#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ReadTiming {
    /// offset in stream of reply data (int)
    pub o: u64,
    /// time since starting to connect when data is received (in milli seconds) (float)
    pub t: f64,
}




