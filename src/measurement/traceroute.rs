use crate::general::{AddressFamily, Protocol, UnixTimestamp};
use crate::measurement::{Measurement, Response};
use crate::serde_utils::{digit_to_bool, skip_empty_in_vec};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::borrow::Cow;

/// https://atlas.ripe.net/docs/apis/result-format/#version-4570
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Traceroute<'a> {
    /// address family, 4 or 6 (integer)
    pub af: AddressFamily,
    /// IP address of the destination (string)
    ///
    /// > Note: Will not be present in cases where traceroute failed to resolve the host name.
    pub dst_addr: Option<Cow<'a, str>>,
    /// name of the destination (string)
    pub dst_name: Cow<'a, str>,
    /// Unix timestamp for end of measurement (int)
    pub endtime: UnixTimestamp,
    /// variation for the Paris mode of traceroute (int)
    ///
    /// > Note: For some reason this value is not always present. The specification says it should
    /// > so I don't know why it gets excluded sometimes. Noted from response with fw 4790.
    pub paris_id: Option<i64>,
    /// "UDP" or "ICMP" (or "TCP", fw >= 4600) (string)
    pub proto: Protocol,
    /// list of hop elements (array)
    pub result: Vec<TraceHop<'a>>,
    /// packet size (int)
    pub size: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum TraceHop<'a> {
    Error {
        error: Cow<'a, str>,
    },
    Result {
        hop: u32,
        #[serde(deserialize_with = "skip_empty_in_vec")]
        result: Vec<Response<'a, TraceReply<'a>>>,
    },
}

impl<'a> TraceHop<'a> {
    pub fn iter_replies<'b>(&'b self) -> impl Iterator<Item = &'b TraceReply<'a>> {
        let responses = match self {
            // Hack solution to avoid returning a dynamic trait type.
            TraceHop::Error { .. } => [].iter(),
            TraceHop::Result { result, .. } => result.iter(),
        };

        responses.filter_map(|x| match x {
            Response::Reply(y) => Some(y),
            _ => None,
        })
    }
}

impl<'a> Response<'a, TraceReply<'a>> {
    pub fn rtt(&self) -> Option<f32> {
        if let Response::Reply(TraceReply { rtt, .. }) = self {
            return match rtt {
                RoundTripTime::OnTime(x) => Some(*x),
                RoundTripTime::Late(_) => None,
            };
        }
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct TraceReply<'a> {
    /// (optional) error ICMP: "N" (network unreachable,), "H" (destination unreachable),
    /// "A" (administratively prohibited), "P" (protocol unreachable), "p" (port unreachable)
    /// (string)
    pub err: Option<ErrorTypes>,
    /// IPv4 or IPv6 source address in reply (string)
    pub from: Cow<'a, str>,
    /// (optional) time-to-live in packet that triggered the error ICMP. Omitted if equal to 1 (int)
    #[serde(skip_serializing_if = "omit_icmp_ttl", default = "icmp_default_ttl")]
    pub ittl: i64,
    #[serde(flatten)]
    pub rtt: RoundTripTime,
    /// (optional) path MTU from a packet too big ICMP (int)
    pub mtu: Option<i64>,
    /// size of reply (int)
    pub size: u64,
    /// time-to-live in reply (int)
    pub ttl: i64,
    /// (optional) TCP flags in the reply packet, for TCP traceroute, concatenated, in the order
    /// 'F' (FIN), 'S' (SYN), 'R' (RST), 'P' (PSH), 'A' (ACK), 'U' (URG) (fw >= 4600) (string)
    pub flags: Option<Cow<'a, str>>,
    /// [optional] information when icmp header is found in reply (object)
    pub icmpext: Option<ICMPHeaderInfo>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ICMPHeaderInfo {
    /// RFC4884 version (int)
    pub version: i64,
    /// "1" if length indication is present, "0" otherwise (int)
    #[serde(with = "digit_to_bool")]
    pub rfc4884: bool,
    /// elements of the object (array)
    pub obj: Vec<ICMPObj>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ICMPObj {
    /// RFC4884 class (int)
    pub class: i64,
    /// RFC4884 type (int)
    pub r#type: i64,
    /// [optional] MPLS data, RFC4950, shown when class is "1" and type is "1" (array)
    pub mpls: Option<Vec<MPLSData>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct MPLSData {
    /// for experimental use (int)
    pub exp: i64,
    /// mpls label (int)
    pub label: i64,
    /// bottom of stack (int)
    pub s: i64,
    /// time to live value (int)
    pub ttl: i64,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum RoundTripTime {
    /// round-trip-time of reply, not present when the response is late (float)
    #[serde(rename = "rtt")]
    OnTime(f32),
    /// (optional) number of packets a reply is late, in this case rtt is not present (int)
    #[serde(rename = "late")]
    Late(u32),
}

impl RoundTripTime {
    pub fn ok(self) -> Option<f32> {
        match self {
            RoundTripTime::OnTime(x) => Some(x),
            RoundTripTime::Late(_) => None,
        }
    }
}

/// Utility functions which specify if the ttl should be excluded from traceroute hop reply details
fn omit_icmp_ttl(ttl: &i64) -> bool {
    *ttl == 1
}

fn icmp_default_ttl() -> i64 {
    1
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ErrorTypes {
    Code(i32),
    Icmp(ICMPError),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ICMPError {
    #[serde(rename = "N")]
    NetworkUnreachable,
    #[serde(rename = "H", alias = "h")] // FIXME: Is "h" really a valid variant? I just guessed.
    DestinationUnreachable,
    #[serde(rename = "A")]
    AdministrativelyProhibited,
    #[serde(rename = "P")]
    ProtocolUnreachable,
    #[serde(rename = "p")]
    PortUnreachable,
}

impl<'a> Measurement<'a, Traceroute<'a>> {
    pub fn iter_route(&self) -> TracerouteIPIter {
        TracerouteIPIter {
            trace: self,
            index: 0,
        }
    }

    pub fn iter_route_with_timeouts(&self) -> impl Iterator<Item = SmallVec<[Cow<str>; 3]>> {
        std::iter::once(smallvec!["placeholder".into()])
            .chain(
                self.iter_route()
                    .map(|x| x.into_iter().map_into().collect()),
            )
            .dedup_with_count()
            .tuple_windows()
            .flat_map(
                |((_, prev), (count, x))| -> Box<dyn Iterator<Item = SmallVec<[Cow<str>; 3]>>> {
                    if !x.is_empty() {
                        Box::new(std::iter::repeat(x).take(count))
                    } else {
                        Box::new((0..count).map(move |n| {
                            smallvec![format!("Timeout {}: {}", n, prev.join(",")).into()]
                        }))
                    }
                },
            )
    }
}

pub struct TracerouteIPIter<'a> {
    trace: &'a Measurement<'a, Traceroute<'a>>,
    index: usize,
}

impl<'a> Iterator for TracerouteIPIter<'a> {
    type Item = SmallVec<[&'a str; 3]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.trace.result.len() + 1 {
            return None;
        }

        self.index += 1;
        if self.index == 1 {
            return Some(smallvec![self.trace.from.as_ref()]);
        }

        if self.index == self.trace.result.len() + 2 {
            return Some(smallvec![self.trace.dst_name.as_ref()]);
        }

        match &self.trace.result[self.index - 2] {
            TraceHop::Error { .. } => Some(SmallVec::new()),
            TraceHop::Result { result, .. } => {
                // Collect ip strings from a given hop
                let mut unique_ips: SmallVec<[&str; 3]> = result
                    .iter()
                    .filter_map(|x| match x {
                        Response::Reply(TraceReply { from, .. }) => Some(from.as_ref()),
                        _ => None,
                    })
                    .collect();

                // Sort and remove duplicate ips from this hop
                unique_ips.sort_unstable();
                unique_ips.dedup();
                Some(unique_ips)
            }
        }
    }
}
