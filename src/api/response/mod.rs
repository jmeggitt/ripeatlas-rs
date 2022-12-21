use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod measurements;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PagedResults<'a, T> {
    count: u64,
    next: Option<Cow<'a, str>>,
    previous: Option<Cow<'a, str>>,
    results: Vec<T>,
}
