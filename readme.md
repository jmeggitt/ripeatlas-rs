# RIPE Atlas API for Rust [WIP]
This library provides serde structures for RIPE Atlas measurements.


## Features
 - `chrono`: When enabled, timestamps will instead be deserialized directly to `DateTime<Utc>` from the [chrono] crate
   instead of integer timestamps.
 - `strict`: This feature enables the serde attribute `deny_unknown_fields` on all measurement structs. This is intended
   to help combat documentation inconsistencies by producing an error on previously unknown fields being provided as
   part of the input. However, this is mostly intended for debugging consistency with the documentation and not it is
   not recommended for regular use as it may produce errors on otherwise valid inputs.


## Documentation Inconsistencies on Measurement Results
One issue that has been challenging to address is that the [official documentation] does not fully address all the
possible responses. The main issue is that the probe measurement code is all written in C and has no idea what JSON is.
Instead, as data is collected it is written to an output stream. This does wonders for efficiency, but also makes it
extremely difficult to enforce adherence to a specific schema and determine all possible outputs without analyzing the
probe code.

I have collected a list of documentation inconsistencies here: https://gist.github.com/jmeggitt/08375285b40e3393da49a261b6b65b52

My approach for this library has been to start by implementing structures and deserialization using [`serde`] based on
the latest firmware version in the official documentation. To test that this library correctly conforms with the
measurement result formats, I test it against actual probe data. The data used in these tests is collected from hourly
data dumps provides by RIPE Atlas.

[official documentation]: https://atlas.ripe.net/docs/apis/result-format/#version-5000
[chrono]: https://crates.io/crates/chrono
