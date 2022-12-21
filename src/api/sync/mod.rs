use crate::api::request::measurements::MeasurementQueryOptions;
use crate::api::response::measurements::Measurement;
use crate::api::response::PagedResults;
use crate::api::UrlEncode;

pub fn get_measurements(
    page: u64,
    options: Option<MeasurementQueryOptions>,
) -> Result<PagedResults<'static, Measurement<'static>>, ureq::Error> {
    let mut url = options.url_encode("https://atlas.ripe.net/api/v2/measurements/");
    url.query_pairs_mut().append_pair("page", &page.to_string());

    let response = ureq::get(url.query().expect("unreachable")).call()?;
    Ok(response.into_json::<PagedResults<Measurement>>()?)
}
