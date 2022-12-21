use crate::api::request::{BoundedParameter, DateFormat, ResponseFormat, SearchCondition};
use crate::api::{Status, UrlEncode};
use crate::general::{AddressFamily, MeasurementType, UnixTimestamp};
use smallvec::SmallVec;
use std::fmt::{Display, Formatter};
use url::form_urlencoded::Serializer;
use url::{form_urlencoded, UrlQuery};

#[derive(Clone, Debug, Default)]
pub struct MeasurementQueryOptions {
    /// Sort the results based on certain field values. Sort directives beginning with '-' imply a
    /// descending/reversed sort order. [id, -id, start_time, -start_time, stop_time, -stop_time,
    /// is_oneoff, -is_oneoff, interval, -interval, type, -type, af, -af, status.name, -status.name,
    /// status.id, -status.id]
    pub sort: Option<SortingOrder>,
    /// filter on ID
    pub id: BoundedParameter<i64>,
    /// filter on ID being one of a comma-separated list
    pub id_in: Vec<i64>,
    /// filter on start time (as a unixtime timestamp) of scheduled measurement.
    pub start_time: BoundedParameter<UnixTimestamp>,
    /// filter on stop time (as a unixtime timestamp) of scheduled measurement.
    pub stop_time: BoundedParameter<UnixTimestamp>,
    /// filter on the interval matching the value.
    pub interval: BoundedParameter<i64>,
    /// return only publicly available measurements.
    pub is_public: bool,
    /// return only one-off measurements.
    pub is_oneoff: bool,
    /// filter on the status of the measurement. Multiple statuses are specified as comma-separated
    /// values. (0: Specified, 1: Scheduled, 2: Ongoing, 4: Stopped, 5: Forced to stop, 6: No
    /// suitable probes, 7: Failed, 8: Archived)
    pub status: SmallVec<[Status; 8]>,
    /// filter on id being less than value
    ///
    /// > **Note:** It is very likely the above documentation from RIPE Atlas is incorrect.
    pub tags: Option<String>,
    /// return all measurements of type value, one of `ping`,`traceroute`, `dns`, `http`, `sslcert`,
    /// `ntp`
    pub r#type: Option<MeasurementType>,
    /// filter on the exact or more specific target IP address
    pub target_ip: Option<String>,
    /// filter by the ID of a probe currently participating in this measurement
    pub current_probes: Option<String>,
    /// filter by the ID of the probes that were selected by participation requests for this
    /// measurement (100 probes max)
    pub participant_logs_probes: Option<Vec<i64>>,
    /// filter on the ASN seen to be routing the target's IP address
    pub target_asn: Option<u32>,
    /// filter on the exact target FQDN (or target ip if fqdn is not available)
    pub target: Option<SearchCondition>,
    /// filter on the exact description
    pub description: Option<SearchCondition>,
    /// return only the fields containing either IPv4 or IPv6 measurements. (4 - IPv4, 6 - IPv6)
    pub af: Option<AddressFamily>,
    /// filter on description (for DNS measurements) or target (other measurements) for containing
    /// value.
    pub search: Option<String>,
    /// filter on the protocol for the measurement, works only it conjunction with
    /// type=dns|traceroute.
    pub protocol: Option<String>,
    /// Filter by group ID
    pub group_id: Option<String>,
    /// Set the number of measurements returned on one page. Maximum size is 500, default is 50.
    pub page_size: Option<u32>,
    /// Used to get the next page of results when the maximum page number has been reached
    pub after: Option<String>,
    /// Choose between available output formats (api, json, jsonp, txt) [api,json,jsonp,txt]
    pub format: ResponseFormat,
    /// Include additional fields named in comma-separated values in response.
    /// [participation_requests,participant_logs,current_probes,is_active,probe_sources,probes]
    pub optional_fields: SmallVec<[OptionalFields; 6]>,
    /// Return fields named in comma-separated values in response, in addition to 'type' and 'id'
    /// which are always present. Optional fields can be listed here also.
    pub fields: Option<String>,
    /// Output datetimes in ISO-8601 format ('iso-8601' or 'json') or as seconds since the epoch
    /// ('unix')
    pub date_format: DateFormat,
    /// An API key to be used to authorize this request
    pub key: Option<String>,
    /// passing in mine=true will return only the measurements of the logged in user
    pub mine: bool,
}

impl UrlEncode for MeasurementQueryOptions {
    fn url_encode_fields(&self, _: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        self.sort.url_encode_fields("sort", pairs);
        self.id.url_encode_fields("id", pairs);
        self.id_in.url_encode_fields("id_in", pairs);
        self.start_time.url_encode_fields("start_time", pairs);
        self.stop_time.url_encode_fields("stop_time", pairs);
        self.interval.url_encode_fields("interval", pairs);
        self.is_public.url_encode_fields("is_public", pairs);
        self.is_oneoff.url_encode_fields("is_oneoff", pairs);
        self.status.url_encode_fields("status", pairs);
        self.tags.url_encode_fields("tags", pairs);
        self.r#type.url_encode_fields("type", pairs);
        self.target_ip.url_encode_fields("target_ip", pairs);
        self.current_probes
            .url_encode_fields("current_probes", pairs);
        self.participant_logs_probes
            .url_encode_fields("participant_logs_probes", pairs);
        self.target_asn.url_encode_fields("target_asn", pairs);
        self.target.url_encode_fields("target", pairs);
        self.description.url_encode_fields("description", pairs);
        self.af.url_encode_fields("af", pairs);
        self.search.url_encode_fields("search", pairs);
        self.protocol.url_encode_fields("protocol", pairs);
        self.group_id.url_encode_fields("group_id", pairs);
        self.page_size.url_encode_fields("page_size", pairs);
        self.after.url_encode_fields("after", pairs);
        self.format.url_encode_fields("format", pairs);
        self.optional_fields
            .url_encode_fields("optional_fields", pairs);
        self.fields.url_encode_fields("fields", pairs);
        // I know this one looks wrong, but it actually need the brackets in the key
        self.date_format
            .url_encode_fields("format[datetime]", pairs);
        self.key.url_encode_fields("key", pairs);
        self.mine.url_encode_fields("mine", pairs);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OptionalFields {
    ParticipationRequests,
    ParticipationLogs,
    CurrentProbes,
    IsActive,
    ProbeSources,
    Probes,
}

impl Display for OptionalFields {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalFields::ParticipationRequests => write!(f, "participation_requests"),
            OptionalFields::ParticipationLogs => write!(f, "participant_logs"),
            OptionalFields::CurrentProbes => write!(f, "current_probes"),
            OptionalFields::IsActive => write!(f, "is_active"),
            OptionalFields::ProbeSources => write!(f, "probe_sources"),
            OptionalFields::Probes => write!(f, "probes"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SortingKey {
    Id,
    StartTime,
    StopTime,
    IsOneoff,
    Interval,
    Type,
    AddressFamily,
    StatusName,
    StatusId,
}

#[derive(Copy, Clone, Debug)]
pub struct SortingOrder {
    key: SortingKey,
    ascending: bool,
}

impl UrlEncode for SortingOrder {
    fn url_encode_fields(
        &self,
        name: &str,
        pairs: &mut form_urlencoded::Serializer<'_, UrlQuery<'_>>,
    ) {
        let mut value = String::new();

        if !self.ascending {
            value.push('-');
        }

        match self.key {
            SortingKey::Id => value.push_str("id"),
            SortingKey::StartTime => value.push_str("start_time"),
            SortingKey::StopTime => value.push_str("stop_time"),
            SortingKey::IsOneoff => value.push_str("is_oneoff"),
            SortingKey::Interval => value.push_str("interval"),
            SortingKey::Type => value.push_str("type"),
            SortingKey::AddressFamily => value.push_str("af"),
            SortingKey::StatusName => value.push_str("status.name"),
            SortingKey::StatusId => value.push_str("status.id"),
        }

        pairs.append_pair(name, &value);
    }
}

impl SortingOrder {
    pub fn new(key: SortingKey, ascending: bool) -> Self {
        SortingOrder { key, ascending }
    }
}

impl Display for SortingOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.ascending {
            write!(f, "-")?;
        }

        match self.key {
            SortingKey::Id => write!(f, "id"),
            SortingKey::StartTime => write!(f, "start_time"),
            SortingKey::StopTime => write!(f, "stop_time"),
            SortingKey::IsOneoff => write!(f, "is_oneoff"),
            SortingKey::Interval => write!(f, "interval"),
            SortingKey::Type => write!(f, "type"),
            SortingKey::AddressFamily => write!(f, "af"),
            SortingKey::StatusName => write!(f, "status.name"),
            SortingKey::StatusId => write!(f, "status.id"),
        }
    }
}
