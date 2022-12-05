use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;
use ripeatlas::measurement::{DnsMeasurement, HttpMeasurement, NtpMeasurement, PingMeasurement, TlsMeasurement};
use crate::common::bzip2::BzipDecoderStream;
use crate::common::{debug_read, debug_read_rayon};

mod common;

#[test]
pub fn test_ntp() {
    perform_test_read::<NtpMeasurement>("ntp-2022-12-01T0000.bz2");
}

#[test]
pub fn test_tls() {
    perform_test_read::<TlsMeasurement>("sslcert-2022-12-01T0000.bz2");
}

#[test]
pub fn test_http() {
    perform_test_read::<HttpMeasurement>("http-2022-12-01T0000.bz2");
}

#[test]
pub fn test_dns() {
    perform_test_read::<DnsMeasurement>("dns-2022-12-01T0000.bz2");
}

#[test]
pub fn test_ping() {
    perform_test_read::<PingMeasurement>("ping-2022-12-01T0000.bz2");
}

fn perform_test_read<T: for<'a> Deserialize<'a>>(data_file: &str) {
    let path = Path::new("../../Downloads").join(data_file);

    let file = BzipDecoderStream::new(path).expect("found and opened input file");
    let mut reader = BufReader::new(file);

    if let Err(e) = debug_read_rayon::<T, _>(&mut reader) {
        panic!("Got read error while handling inputs: {}", e)
    }
}
