use crate::general::{AddressFamily, Protocol};
use crate::serde_utils::one_or_many;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Dns<'a> {
    /// [optional] IP version: "4" or "6" (int)
    af: Option<AddressFamily>,
    // /// [optional] instance ID for a collection of related measurement results (int)
    // bundle: Option<i64>,
    dst_addr: Option<Cow<'a, str>>,
    dst_name: Option<Cow<'a, str>>,
    dst_port: Option<Cow<'a, str>>,
    error: Option<DNSLookupError<'a>>,
    proto: Option<Protocol>,
    qbuf: Option<Cow<'a, str>>,
    result: Option<DNSResponse<'a>>,
    /// TODO:
    resultset: Option<Vec<Value>>,
    retry: Option<u32>,
    subid: Option<i64>,
    submax: Option<u32>,

    name: Option<Cow<'a, str>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct DNSResponse<'a> {
    /// answer count, RFC 1035 4.1.1 (int)
    #[serde(rename = "ANCOUNT")]
    answer_count: u32,
    /// additional record count, RFC 1035, 4.1.1 (int)
    #[serde(rename = "ARCOUNT")]
    additional_record_count: u32,
    /// query ID, RFC 1035 4.1.1 (int)
    #[serde(rename = "ID")]
    id: i64,
    /// name server count (int)
    #[serde(rename = "NSCOUNT")]
    name_server_count: u32,
    /// number of queries (int)
    #[serde(rename = "QDCOUNT")]
    number_of_queries: u32,
    /// answer payload buffer from the server, base64 encoded (string) See example code for decoding
    /// the value
    abuf: Cow<'a, str>,
    /// first two records from the response decoded by the probe, if they are TXT or SOA; other RR
    /// can be decoded from "abuf" (array of objects)
    answers: Option<Vec<DNSRecord<'a>>>,
    /// [optional] response time in milli seconds (float)
    rt: Option<f32>,
    /// [optional] response size (int)
    size: Option<u64>,
    /// [optional] TTL (hop limit for IPv6) field from UDP reply packet (from 5010) (int)
    ttl: Option<u32>,
    qt: Option<f32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "TYPE", rename_all = "UPPERCASE")]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum DNSRecord<'a> {
    #[serde(rename_all = "UPPERCASE")]
    Txt {
        // mname: Option<Cow<'a, str>>,
        name: Cow<'a, str>,
        #[serde(deserialize_with = "one_or_many")]
        rdata: Vec<Cow<'a, str>>,
    },
    #[serde(rename_all = "UPPERCASE")]
    Soa {
        mname: Cow<'a, str>,
        name: Cow<'a, str>,
        rname: Cow<'a, str>,
        serial: i64,
        ttl: i64,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum DNSLookupError<'a> {
    Timeout { timeout: u64 },
    Other(HashMap<Cow<'a, str>, Cow<'a, str>>),
}
