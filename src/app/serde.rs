use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}

pub fn deserialize_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    let sn: StringOrNumber<_> = StringOrNumber::deserialize(deserializer)?;
    match sn {
        StringOrNumber::String(s) => s.parse::<T>().map_err(D::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}
