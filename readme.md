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

## Goals and API Support
 - Measurement Result Types
   - [x] Ping
   - [x] Traceroute
   - [x] DNS Lookup
   - [x] HTTP
   - [x] NTP
   - [x] TLS (SSL) GET Cert
   - [ ] Wifi
      > **Note:** This measurement type was disconnected in firmware release 5000 and implementation is not planned unless
     I receive a request to support this type of measurement
 - API Support
   - [`anchor-measrements`]
     - [ ] `GET /api/v2/anchor-measurements/{pk}/`
     - [ ] `GET /api/v2/anchor-measurements/` **[Medium priority]**
   - [`anchors`]
     - [ ] `GET /api/v2/anchors/{pk}/`
     - [ ] `GET /api/v2/anchors/` **[Medium priority]**
   - [`credits`]
     - [ ] `GET /api/v2/credits/` **[Low priority]**
     - [ ] `GET /api/v2/credits/income-items/`
     - [ ] `GET /api/v2/credits/expense-items/`
     - [ ] `POST /api/v2/credits/transfers/`
     - [ ] `GET /api/v2/credits/voucher/{code}`
     - [ ] `POST /api/v2/credits/voucher/redeem/`
     - [ ] `GET /api/v2/credits/standing-order/`
     - [ ] `POST /api/v2/credits/standing-order/`
     - [ ] `GET /api/v2/credits/standing-order/{pk}/`
     - [ ] `PATCH /api/v2/credits/standing-order/{pk}/`
     - [ ] `DELETE /api/v2/credits/standing-order/{pk}/`
     - [ ] `GET /api/v2/credits/bill-me/`
     - [ ] `POST /api/v2/credits/bill-me/`
     - [ ] `GET /api/v2/credits/bill-me/{pk}/`
     - [ ] `PATCH /api/v2/credits/bill-me/{pk}/`
     - [ ] `DELETE /api/v2/credits/bill-me/{pk}/`
     - [ ] `GET /api/v2/credits/transactions/`
     - [ ] `GET /api/v2/credits/members/`
     - [ ] `POST /api/v2/credits/members/claim/`
   - [`keys`]
     - [ ] `GET /api/v2/keys/permissions/`
     - [ ] `GET /api/v2/keys/permissions/{permission}/targets/`
     - [ ] `GET /api/v2/keys/{uuid}/`
     - [ ] `PUT /api/v2/keys/{uuid}/`
     - [ ] `DELETE /api/v2/keys/{uuid}/`
     - [ ] `GET /api/v2/keys/`
     - [ ] `POST /api/v2/keys/`
   - [`measrements`]
     - [ ] `GET /api/v2/measurements/` **[High priority]**
     - [ ] `POST /api/v2/measurements/` **[Medium priority]**
     - [ ] `GET /api/v2/measurements/{pk}/` **[Medium priority]**
     - [ ] `PATCH /api/v2/measurements/{pk}/`
     - [ ] `DELETE /api/v2/measurements/{pk}/`
     - [ ] `GET /api/v2/measurements/{pk}/results/` **[High priority]**
     - [ ] `GET /api/v2/measurements/{pk}/latest/` **[High priority]**
     - [ ] `GET /api/v2/measurements/my/`  **[Medium priority]**
     - [ ] `GET /api/v2/measurements/ping/`
     - [ ] `GET /api/v2/measurements/traceroute/`
     - [ ] `GET /api/v2/measurements/sslcert/`
     - [ ] `GET /api/v2/measurements/ntp/`
     - [ ] `GET /api/v2/measurements/wifi/`
     - [ ] `GET /api/v2/measurements/http/`
     - [ ] `GET /api/v2/measurements/dns/`
     - [ ] `GET /api/v2/measurements/groups/`
     - [ ] `GET /api/v2/measurements/groups/{pk}/`
     - [ ] `DELETE /api/v2/measurements/groups/{pk}/`
     - [ ] `GET /api/v2/measurements/tags/`
     - [ ] `GET /api/v2/measurements/tags/{tag}/results/`
     - [ ] `GET /api/v2/measurements/my-tags/`
     - [ ] `GET /api/v2/measurements/my-tags/{tag}/`
     - [ ] `POST /api/v2/measurements/my-tags/{tag}/stop/`
     - [ ] `GET /api/v2/measurements/{pk}/tags/`
     - [ ] `POST /api/v2/measurements/{pk}/tags/`
     - [ ] `GET /api/v2/measurements/{pk}/tags/{tag}/`
     - [ ] `DELETE /api/v2/measurements/{pk}/tags/{tag}/`
     - [ ] `GET /api/v2/measurements/{msm_id}/participation-requests/`
     - [ ] `POST /api/v2/measurements/{msm_id}/participation-requests/`
     - [ ] `GET /api/v2/measurements/{msm_id}/participation-requests/{source_id}/`
     - [ ] `GET /api/v2/measurements/{pk}/private/`
   - [`participation-requests`]
     - [ ] `GET /api/v2/participation-requests/{source_id}/` **[Medium priority]**
   - [`probes`]
     - [ ] `GET /api/v2/probes/` **[High priority]**
     - [ ] `GET /api/v2/probes/{pk}/` **[Medium priority]**
     - [ ] `PUT /api/v2/probes/{pk}/` **[Medium priority]**
     - [ ] `PATCH /api/v2/probes/{pk}/`
     - [ ] `GET /api/v2/probes/{prb_id}/measurements/` **[Medium priority]**
     - [ ] `GET /api/v2/probes/archive/`
     - [ ] `GET /api/v2/probes/rankings/`
     - [ ] `GET /api/v2/probes/tags/`
     - [ ] `GET /api/v2/probes/tags/{slug}/`

   
[official documentation]: https://atlas.ripe.net/docs/apis/result-format/#version-5000
[chrono]: https://crates.io/crates/chrono
[`anchor-measrements`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#anchor-measurements
[`anchors`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#anchors
[`credits`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#credits
[`keys`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#keys
[`measrements`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#measurements
[`participation-requests`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#participation-requests
[`probes`]: https://atlas.ripe.net/docs/apis/rest-api-reference/#probes