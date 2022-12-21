use crate::api::UrlEncode;
use std::ops::Bound;
use url::form_urlencoded::Serializer;
use url::UrlQuery;

pub mod measurements;

#[derive(Debug, Copy, Clone)]
pub enum DateFormat {
    Unix,
    Json,
    Iso8601,
}

impl UrlEncode for DateFormat {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        match self {
            DateFormat::Unix => {} // default
            DateFormat::Json => {
                pairs.append_pair(name, "json");
            }
            DateFormat::Iso8601 => {
                pairs.append_pair(name, "iso-8601");
            }
        }
    }
}

impl Default for DateFormat {
    fn default() -> Self {
        DateFormat::Unix
    }
}

#[derive(Debug, Clone)]
pub enum ResponseFormat {
    API,
    JSON,
    JSONP { callback: String },
    TXT,
}

impl UrlEncode for ResponseFormat {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        match self {
            ResponseFormat::API => {
                pairs.append_pair(name, "api");
            }
            ResponseFormat::JSON => {
                pairs.append_pair(name, "json");
            }
            ResponseFormat::JSONP { callback } => {
                pairs.append_pair(name, "jsonp");
                pairs.append_pair("callback", callback);
            }
            ResponseFormat::TXT => {
                pairs.append_pair(name, "txt");
            }
        }
    }
}

impl Default for ResponseFormat {
    fn default() -> Self {
        // TODO: Maybe switch to TXT?
        ResponseFormat::JSON
    }
}

#[derive(Debug, Clone)]
pub enum SearchCondition {
    Exact(String),
    Partial {
        contains: Option<String>,
        starts_with: Option<String>,
        ends_with: Option<String>,
    },
}

impl UrlEncode for SearchCondition {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        match self {
            SearchCondition::Exact(value) => {
                pairs.append_pair(name, value);
            }
            SearchCondition::Partial {
                contains,
                starts_with,
                ends_with,
            } => {
                if let Some(value) = contains {
                    pairs.append_pair(&format!("{}__contains", name), value);
                }

                if let Some(value) = starts_with {
                    pairs.append_pair(&format!("{}__startswith", name), value);
                }

                if let Some(value) = ends_with {
                    pairs.append_pair(&format!("{}__endswith", name), value);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoundedParameter<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<T: Eq + ToString> UrlEncode for BoundedParameter<T> {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        if self.lower == self.upper {
            if let Bound::Included(x) = &self.lower {
                pairs.append_pair(name, &x.to_string());
                return;
            }
        }

        match &self.lower {
            Bound::Included(x) => {
                pairs.append_pair(&format!("{}__gte", name), &x.to_string());
            }
            Bound::Excluded(x) => {
                pairs.append_pair(&format!("{}__gt", name), &x.to_string());
            }
            Bound::Unbounded => {}
        }

        match &self.upper {
            Bound::Included(x) => {
                pairs.append_pair(&format!("{}__lte", name), &x.to_string());
            }
            Bound::Excluded(x) => {
                pairs.append_pair(&format!("{}__lt", name), &x.to_string());
            }
            Bound::Unbounded => {}
        }
    }
}

impl<T> Default for BoundedParameter<T> {
    fn default() -> Self {
        BoundedParameter {
            lower: Bound::Unbounded,
            upper: Bound::Unbounded,
        }
    }
}
