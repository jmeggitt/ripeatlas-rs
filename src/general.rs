use crate::api::UrlEncode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::form_urlencoded::Serializer;
use url::UrlQuery;

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum AddressFamily {
    IPv4 = 4,
    IPv6 = 6,
}

impl UrlEncode for AddressFamily {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        match self {
            AddressFamily::IPv4 => pairs.append_pair(name, "4"),
            AddressFamily::IPv6 => pairs.append_pair(name, "6"),
        };
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Protocol {
    UDP,
    TCP,
    ICMP,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(rename = "lowercase")]
pub enum MeasurementType {
    Ping,
    Traceroute,
    Dns,
    SslCert,
    Http,
    Ntp,
    Wifi,
}

impl UrlEncode for MeasurementType {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        match self {
            MeasurementType::Ping => pairs.append_pair(name, "ping"),
            MeasurementType::Traceroute => pairs.append_pair(name, "traceroute"),
            MeasurementType::Dns => pairs.append_pair(name, "dns"),
            MeasurementType::SslCert => pairs.append_pair(name, "sslcert"),
            MeasurementType::Http => pairs.append_pair(name, "http"),
            MeasurementType::Ntp => pairs.append_pair(name, "ntp"),
            MeasurementType::Wifi => pairs.append_pair(name, "wifi"),
        };
    }
}

#[cfg(not(feature = "chrono"))]
pub type UnixTimestamp = i64;

#[cfg(feature = "chrono")]
pub type UnixTimestamp = chrono::DateTime<chrono::Utc>;
