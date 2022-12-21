use crate::api::Status;
use crate::general::{AddressFamily, Protocol, UnixTimestamp};
use crate::measurement::http;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Measurement<'a> {
    /// Indicates this measurement is publicly available; may be changed from false to true, but not
    /// from true to false
    pub is_public: bool,
    /// User-defined description of the measurement
    pub description: Option<Cow<'a, str>>,
    /// The unique identifier that RIPE Atlas assigned to this measurement
    pub id: u64,
    /// The URL that contains the results of this measurement
    pub result: Cow<'a, str>,
    /// The ID of the measurement group. This ID references a measurement acting as group master
    pub group_id: Option<i64>,
    /// [ 4, 6 ] [Not for wifi] IPv4 of IPv6 Address family of the measurement
    pub af: Option<AddressFamily>,
    /// Indicates this is a one-off or a recurring measurement
    pub is_oneoff: bool,
    /// Distribution of probes' measurements throughout the interval (default is half the interval,
    /// maximum 400 seconds)
    pub spread: Option<u64>,
    /// Indicates that a name should be resolved (using DNS) on the probe. Otherwise it will be
    /// resolved on the RIPE Atlas servers
    pub resolve_on_probe: bool,
    /// Configured start time (as a unix timestamp)
    pub start_time: UnixTimestamp,
    /// Actual end time of measurement (as a unix timestamp)
    pub stop_time: Option<UnixTimestamp>,
    // /// [ "ping", "traceroute", "dns", "sslcert", "http", "ntp", "wifi" ] The type of the
    // /// measurement
    // pub r#type: MeasurementType,
    /// Current status of the measurement. Status can be: Specified, Scheduled, Ongoing, Stopped,
    /// Forced to stop, No suitable probes, Failed or Archived
    pub status: MeasurementStatus,
    /// Indicates if all probe requests have made it through the scheduling process
    pub is_all_scheduled: bool,
    /// Indicates this measurement is a reachability test
    pub is_reachability_test: Option<bool>,
    /// Number of participating probes
    pub participant_count: Option<u64>,
    /// The number of the Autonomous System the IP address of the target belongs to
    pub target_asn: Option<u32>,
    /// Enclosing prefix of the IP address of the target
    pub target_prefix: Option<Cow<'a, str>>,
    /// The IP Address of the target of the measurement
    pub target_ip: Option<Cow<'a, str>>,
    /// The creation date and time of the measurement (Defaults to unix timestamp format)
    pub creation_time: UnixTimestamp,
    /// Indicates this measurement belongs to a wifi measurement group
    pub in_wifi_group: bool,
    /// The list of IP addresses returned for the fqdn in the `target` field by the backend
    /// infra-structure resolvers
    pub resolved_ips: Option<Vec<Cow<'a, str>>>,
    /// Number of probes requested, but not necessarily granted to this measurement
    pub probes_requested: Option<i64>,
    /// Number of probes actually scheduled for this measurement
    pub probes_scheduled: Option<u64>,
    /// The API URL of the measurement group.
    pub group: Option<Cow<'a, str>>,
    /// probes involved in this measurement
    pub probes: Option<Vec<u64>>,
    /// estimated RIPE Atlas credits consumed by this measurement per day
    /// > **Note:** For some reason this value can be negative.
    pub estimated_results_per_day: i64,
    pub credits_per_result: u64,
    #[serde(skip)]
    pub probe_sources: Vec<serde::de::IgnoredAny>,
    #[serde(skip)]
    pub participation_requests: Vec<serde::de::IgnoredAny>,
    /// The user ID of the owner of this measurement
    pub user_id: Option<i64>,
    /// The ID of the user who will be billed for this measurement
    pub bill_to_user_id: Option<i64>,
    /// Array of tags to apply to the measurement
    pub tags: Vec<Cow<'a, str>>,
    #[serde(flatten)]
    pub config: MeasurementSpecific<'a>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MeasurementStatus {
    id: u32,
    name: Status,
    when: Option<UnixTimestamp>,
}

/// Nearly every single field is optional since requests will frequently return many of these fields
/// as null.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MeasurementSpecific<'a> {
    Ping {
        /// The number of packets send in a measurement execution. Value must be between 1 and 16.
        /// Default is 3
        packets: Option<u64>,
        /// size of the data part of the packet, i.e. excluding any IP and ICMP headers. Value must
        /// be between 1 and 2048
        size: Option<u64>,
        /// Time between packets in milliseconds. Value must be between 2 and 300000
        packet_interval: Option<u64>,
        /// Include the probe ID (encoded as ASCII digits) as part of the payload
        include_probe_id: Option<bool>,
    },
    Traceroute {
        /// The target port number (TCP only). Defaults to 80
        port: Option<u16>,
        /// The number of packets send in a measurement execution. Value must be between 1 and 16.
        /// Default is 3
        packets: Option<u64>,
        /// TTL (time to live) of the first hop
        first_hop: Option<u8>,
        /// Traceroute measurement stops after the hop at which the TTL reaches this value
        max_hops: Option<u8>,
        /// The number of paris traceroute variations to try. Zero disables paris traceroute. Value
        /// must be between 0 and 64
        paris: Option<u8>,
        /// size of the data part of the packet, i.e. excluding any IP, ICMP, UDP or TCP headers.
        /// Value must be between 0 and 2048
        size: Option<u64>,
        /// [ "ICMP", "UDP", "TCP" ] Protocol used in measurement
        protocol: Option<Protocol>,
        /// Response timeout for one packet
        response_timeout: Option<i64>,
        /// Time to wait (in milliseconds) for a duplicate response after receiving the first
        /// response
        duplicate_timeout: Option<u64>,
        /// Size of an IPv6 hop-by-hop option header filled with NOPs
        hop_by_hop_option_size: Option<u64>,
        /// Size of an IPv6 destination option header filled with NOPs
        destination_option_size: Option<u64>,
        /// Do not fragment outgoing packets
        dont_fragment: Option<bool>,
        /// The traffic class (IPv6) or type of service and precedence (IPv4) value
        traffic_class: Option<i64>,
    },
    Dns {
        /// Set the EDNS0 option for UDP payload size to this value, between 512 and 4096. Defaults
        /// to 512)
        udp_payload_size: Option<u64>,
        /// Send the DNS query to the probe's local resolvers (instead of an explicitly specified
        /// target)
        use_probe_resolver: Option<bool>,
        /// Indicates Recursion Desired bit was set
        set_rd_bit: Option<bool>,
        /// Each probe prepends its probe number and a timestamp to the DNS query argument to make
        /// it unique
        prepend_probe_id: Option<bool>,
        /// [ "UDP", "TCP" ] Protocol used in measurement. Defaults to UDP
        protocol: Protocol,
        /// Number of times to retry
        retry: Option<u64>,
        /// include the raw DNS query data in the result. Defaults to false
        include_qbuf: Option<bool>,
        /// Indicates Name Server Identifier (RFC5001) was set
        set_nsid_bit: Option<bool>,
        /// include the raw DNS answer data in the result. Defaults to true
        include_abuf: bool,
        /// [ "IN", "CHAOS" ] The `class` part of the query used in the measurement
        query_class: Option<DnsQueryClass>,
        /// The `argument` part of the query used in the measurement
        query_argument: Option<Cow<'a, str>>,
        /// [ "A", "AAAA", "ANY", "CNAME", "DNSKEY", "DS", "MX", "NS", "NSEC", "PTR", "RRSIG",
        /// "SOA", "TXT", "SRV", "NAPTR", "TLSA" ] The `type` part of the query used in the
        /// measurement
        query_type: Option<DnsQueryType>,
        /// Indicates DNSSEC Checking Disabled (RFC4035) was set
        set_cd_bit: Option<bool>,
        /// Indicates DNSSEC OK (RFC3225) was set
        set_do_bit: Option<bool>,
        /// Allow the use of $p (probe ID), $r (random 16-digit hex string) and $t (timestamp) in
        /// the query_argument
        use_macros: Option<bool>,
        /// Timeout in milliseconds (default: 5000)
        timeout: Option<u64>,
        /// Enable DNS over Transport Layer Security (RFC7858)
        tls: Option<bool>,
        /// UDP or TCP port, if not specified defaults to port 53 or to port 853 for DNS-over-TLS
        port: Option<u16>,
        /// Enable an EDNS Client Subnet (RFC7871) of 0.0.0.0/0 0 or ::/0
        default_client_subnet: Option<bool>,
        /// Insert client cookie in requests and process server cookies
        cookies: Option<bool>,
        /// Report the IP time-to-live field (hop limit for IPv6) of DNS reply packets received
        /// (only for UDP)
        ttl: Option<bool>,
    },
    Http {
        /// Enable time-to-resolve, time-to-connect and time-to-first-byte measurements
        extended_timing: Option<bool>,
        /// Include fields added by extended_timing and adds readtiming which reports for each read
        /// system call when it happened and how much data was delivered
        more_extended_timing: Option<bool>,
        /// Maximum number of bytes in the reponse header, defaults to 0
        header_bytes: Option<u64>,
        /// [ "GET", "POST", "HEAD" ] http verb of the measurement request
        method: Option<http::Method>,
        /// Path of the requested URL
        path: Cow<'a, str>,
        /// Optional query parameters of the requested URL
        query_string: Cow<'a, str>,
        /// [ "RIPE Atlas: https://atlas.ripe.net/" ] user agent header field sent in the http
        /// request. Always set to 'RIPE Atlas: https//atlas.ripe.net'
        user_agent: Option<Cow<'a, str>>,
        max_bytes_read: Option<u64>,
        /// [ "1.0", "1.1" ] http version of measurement request
        version: Option<Cow<'a, str>>,
        /// The target port number Defaults to 80 (HTTP) or 443 (HTTPS)
        port: Option<u16>,
        /// Value for the Host header if different than the target
        host: Option<Cow<'a, str>>,
        /// Use HTTPS instead of plaintext HTTP
        https: Option<bool>,
    },
    Ntp {
        /// The number of packets send in a measurement execution. Value must be between 1 and 16.
        /// Default is 3
        packets: u64,
        /// Per packet timeout in milliseconds
        timeout: u64,
    },
    SslCert {
        /// The target port number. Defaults to 443
        port: Option<u16>,
        /// Server Name Indication (SNI) hostname
        hostname: Option<Cow<'a, str>>,
    },
    Wifi {
        /// Indicates IPv4 measurements are attempted in this group
        ipv4: bool,
        /// Indicates IPv6 measurements are attempted in this group
        ipv6: bool,
        /// Certificate in PEM format
        cert: Cow<'a, str>,
        /// Wait this amount of time before executing measurement commands.
        extra_wait: u64,
        /// [ "eduroam", "guestnet", "ripemtg-2.4-74", "ripemtg-nat64-2.4-74", "phicoh-test-2.4" ]
        /// Wifi SSID to connect to. Max. 32 characters
        ssid: Cow<'a, str>,
        /// [ "WPA-PSK", "WPA-EAP" ] Authentication mechanism used for the wifi connection. For
        /// WPA-PSK `psk` field is also required,for WPA-EAP `eap` and `password` fields are
        /// required
        key_mgmt: Cow<'a, str>,
        /// [ "TTLS", "PEAP" ] Extensible Authentication Protocol type. Currently only `TTLS` is
        /// available
        eap: Cow<'a, str>,
        /// Username used for wifi connection. Used for both outer and inner connection if
        /// anonymous_identity is omitted
        identity: Cow<'a, str>,
        /// Username used for outer connection. If omitted the `identity` field is used for the
        /// outer connection
        anonymous_identity: Cow<'a, str>,
        /// [ "auth=EAP-MSCHAPV2", "auth=MSCHAPV2", "auth=PAP" ] Connection and Authentication
        /// directives for the inner connection. Only used for WPA-EAP. Currently only EAP-MSCHAPv2
        /// is available
        phase2: Cow<'a, str>,
        /// Indicates that BSSID radio signal strength will be measured and stored
        rssi: bool,
    },
    #[serde(rename = "probe connection log")]
    ProbeConnectionLog,
    Traffic,
}

#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum DnsQueryClass {
    IN,
    CHAOS,
}

#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum DnsQueryType {
    A,
    AAA,
    AAAA,
    ANY,
    CNAME,
    DNSKEY,
    DS,
    MX,
    NS,
    NSEC,
    PTR,
    RRSIG,
    SOA,
    TXT,
    SRV,
    NAPTR,
    TLSA,
}
