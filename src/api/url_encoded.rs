use smallvec::SmallVec;
use url::form_urlencoded::Serializer;
use url::{Url, UrlQuery};

pub trait UrlEncode {
    fn url_encode(&self, base_url: &str) -> Url {
        let mut url = Url::parse(base_url).expect("valid base url");
        self.url_encode_fields("", &mut url.query_pairs_mut());
        url
    }

    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>);
}

impl<T: UrlEncode> UrlEncode for Option<T> {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        if let Some(this) = self {
            this.url_encode_fields(name, pairs);
        }
    }
}

impl UrlEncode for bool {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        if *self {
            pairs.append_pair(name, "true");
        }
    }
}

impl UrlEncode for String {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        pairs.append_pair(name, self);
    }
}

impl UrlEncode for u32 {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        pairs.append_pair(name, &self.to_string());
    }
}

impl<T: ToString> UrlEncode for Vec<T> {
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        let mut value = String::new();

        for item in self {
            if !value.is_empty() {
                value.push(',');
            }

            value.push_str(&item.to_string());
        }

        pairs.append_pair(name, &value);
    }
}

impl<T: ToString, const N: usize> UrlEncode for SmallVec<[T; N]>
where
    [T; N]: smallvec::Array<Item = T>,
{
    fn url_encode_fields(&self, name: &str, pairs: &mut Serializer<'_, UrlQuery<'_>>) {
        let mut value = String::new();

        for item in self {
            if !value.is_empty() {
                value.push(',');
            }

            value.push_str(&item.to_string());
        }

        pairs.append_pair(name, &value);
    }
}
