//! Assorted utility function to assist in serializing and deserializing data for some of the
//! stranger edge cases.
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;

#[derive(Deserialize)]
#[serde(untagged)]
enum PossiblyEmpty<A> {
    Nonempty(A),
    Empty {},
}

struct ItemVisitor<'a, A> {
    _phantom: PhantomData<&'a A>,
}

impl<'de, 'a: 'de, A> Visitor<'de> for ItemVisitor<'a, A>
where
    A: Deserialize<'de>,
{
    type Value = Vec<A>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of objects")
    }

    fn visit_seq<V>(self, mut access: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = access.next_element::<PossiblyEmpty<A>>()? {
            if let PossiblyEmpty::Nonempty(value) = item {
                items.push(value);
            }
        }

        Ok(items)
    }
}

/// Deserialize a `Vec<T>` while skipping empty objects. For example `[1,2,{},3]` in JSON would be
/// treated as `[1,2,3]`.
pub fn skip_empty_in_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + 'de,
{
    deserializer.deserialize_seq(ItemVisitor {
        _phantom: PhantomData,
    })
}

pub mod digit_to_bool {
    use serde::de::{Error, Deserialize, Deserializer};
    use serde::Serializer;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(Error::unknown_variant(&x.to_string(), &["0", "1"])),
        }
    }

    pub fn serialize<S>(this: &bool, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match this {
            false => serializer.serialize_u8(0),
            true => serializer.serialize_u8(1),
        }
    }
}

pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum OneOrMany<T> {
        One(T),
        Many(Vec<T>),
    }

    match OneOrMany::<T>::deserialize(deserializer)? {
        OneOrMany::One(x) => Ok(vec![x]),
        OneOrMany::Many(x) => Ok(x),
    }
}
