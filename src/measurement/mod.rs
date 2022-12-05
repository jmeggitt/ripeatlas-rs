use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

mod general;

pub mod traceroute;
pub mod dns;
pub mod ping;
pub mod http;
pub mod ntp;
pub mod tls;

pub use general::*;


pub type TracerouteMeasurement<'a> = Measurement<'a, traceroute::Traceroute<'a>>;
pub type DnsMeasurement<'a> = Measurement<'a, dns::Dns<'a>>;
pub type PingMeasurement<'a> = Measurement<'a, ping::Ping<'a>>;
pub type HttpMeasurement<'a> = Measurement<'a, http::Http<'a>>;
pub type NtpMeasurement<'a> = Measurement<'a, ntp::Ntp<'a>>;
pub type TlsMeasurement<'a> = Measurement<'a, tls::Tls<'a>>;


#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct Measurement<'a, T> {
    #[serde(flatten)]
    inner: T,
    pub fw: u32,
    /// IP address of the probe as know by controller (string)
    pub from: Cow<'a, str>,
    /// IP address of the probe as know by controller (string)
    pub group_id: Option<i64>,
    /// last time synchronised. How long ago (in seconds) the clock of the probe was found to be in
    /// sync with that of a controller. The value -1 is used to indicate that the probe does not
    /// know whether it is in sync (int)
    ///
    /// > Note: This value may not be available for systems with firmware prior to version 4749.
    pub lts: Option<i64>,
    /// measurement identifier (int)
    pub msm_id: i64,
    /// measurement type (string)
    pub msm_name: Cow<'a, str>,
    /// source probe ID (int)
    pub prb_id: i64,
    pub src_addr: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "chrono", serde(with = "chrono::serde::ts_seconds"))]
    pub timestamp: UnixTimestamp,
    /// The type of the measurement (string)
    pub r#type: Cow<'a, str>,
    // TODO: ttr might be a valid candidate
    /// Since Version 5000, measurement results contain a separate field, called "mver", that
    /// specifies the version of the measurement code. This field has the format "x.y.z", where the
    /// "x" field is that major version, which changes when the measurement results are incompatible
    /// with the previous version. The "y" field is the minor version, and changes when new fields
    /// are added, but otherwise old parsers can still parse measurement results. Finally, the "z"
    /// field specifies that the measurement code changed, but the output format is still the same.
    /// This happens when only (minor) bugs are fixed and no new features are added.
    pub mver: Option<Cow<'a, str>>,
    /// [optional] instance ID for a collection of related measurement results (int)
    pub bundle: Option<i64>,
    /// time to resolve dst_name in milliseconds (float)
    pub ttr: Option<f64>,
}

impl<'a, T> Deref for Measurement<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


