pub mod request;
pub mod response;

#[cfg(feature = "sync")]
pub mod sync;

mod url_encoded;
pub use url_encoded::UrlEncode;

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Status {
    Specified = 0,
    Scheduled = 1,
    Ongoing = 2,
    Stopped = 4,
    #[serde(rename = "Forced to Stop")]
    ForcedToStop = 5,
    #[serde(rename = "No suitable probes")]
    NoSuitableProbes = 6,
    Failed = 7,
    Archived = 8,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        (*self as u8).to_string()
    }
}
