//! Strict JSON for frozen fixtures: objects keep their field order, duplicate
//! fields are errors, and integers remain distinct from decimal numbers.
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeMap, SerializeSeq, Serializer},
};
use std::{fmt, fs, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Json {
    Null,
    Boolean(bool),
    Integer(i128),
    Decimal(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn parse(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|error| error.to_string())
    }
    pub fn read(path: &Path) -> Result<Self, String> {
        let text =
            fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
        Self::parse(&text).map_err(|error| format!("{}: {error}", path.display()))
    }
    pub fn get(&self, name: &str) -> Option<&Json> {
        self.as_object()?
            .iter()
            .find(|(field, _)| field == name)
            .map(|(_, value)| value)
    }
    pub fn as_object(&self) -> Option<&[(String, Json)]> {
        match self {
            Self::Object(fields) => Some(fields),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&[Json]> {
        match self {
            Self::Array(values) => Some(values),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_integer(&self) -> Option<i128> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }
    /// An integer or a decimal, as a finite number of points or bytes.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Integer(value) => Some(*value as f64),
            Self::Decimal(value) => Some(*value),
            _ => None,
        }
    }
    pub fn field_names(&self) -> Option<Vec<&str>> {
        Some(
            self.as_object()?
                .iter()
                .map(|(name, _)| name.as_str())
                .collect(),
        )
    }
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StrictVisitor)
    }
}

struct StrictVisitor;

impl<'de> Visitor<'de> for StrictVisitor {
    type Value = Json;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON value")
    }
    fn visit_unit<E>(self) -> Result<Json, E> {
        Ok(Json::Null)
    }
    fn visit_bool<E>(self, value: bool) -> Result<Json, E> {
        Ok(Json::Boolean(value))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Json, E> {
        Ok(Json::Integer(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Json, E> {
        Ok(Json::Integer(value.into()))
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Json, E> {
        if value.is_finite() {
            Ok(Json::Decimal(value))
        } else {
            Err(E::custom("non-finite JSON number"))
        }
    }
    fn visit_str<E>(self, value: &str) -> Result<Json, E> {
        Ok(Json::String(value.to_owned()))
    }
    fn visit_string<E>(self, value: String) -> Result<Json, E> {
        Ok(Json::String(value))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Json, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element()? {
            values.push(value);
        }
        Ok(Json::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Json, A::Error> {
        let mut fields: Vec<(String, Json)> = Vec::new();
        while let Some(name) = map.next_key::<String>()? {
            if fields.iter().any(|(field, _)| *field == name) {
                return Err(de::Error::custom(format!("duplicate JSON field: {name}")));
            }
            fields.push((name, map.next_value()?));
        }
        Ok(Json::Object(fields))
    }
}

pub(crate) fn string(value: impl ToString) -> Json {
    Json::String(value.to_string())
}

pub(crate) fn object<const N: usize>(fields: [(&str, Json); N]) -> Json {
    Json::Object(
        fields
            .into_iter()
            .map(|(name, value)| (name.to_owned(), value))
            .collect(),
    )
}

/// Replaces a report field in place, so that the field keeps its first position.
pub(crate) fn set(fields: &mut Vec<(String, Json)>, name: &str, value: Json) {
    match fields.iter_mut().find(|(field, _)| field == name) {
        Some(field) => field.1 = value,
        None => fields.push((name.to_owned(), value)),
    }
}

impl Serialize for Json {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Json::Null => serializer.serialize_unit(),
            Json::Boolean(value) => serializer.serialize_bool(*value),
            Json::Integer(value) => serializer.serialize_i128(*value),
            Json::Decimal(value) => serializer.serialize_f64(*value),
            Json::String(value) => serializer.serialize_str(value),
            Json::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            Json::Object(fields) => {
                let mut map = serializer.serialize_map(Some(fields.len()))?;
                for (name, value) in fields {
                    map.serialize_entry(name, value)?;
                }
                map.end()
            }
        }
    }
}

pub(crate) fn pretty(value: &Json) -> String {
    serde_json::to_string_pretty(value).expect("strict JSON values serialize")
}

pub(crate) fn compact(value: &Json) -> String {
    serde_json::to_string(value).expect("strict JSON values serialize")
}

#[cfg(test)]
mod tests {
    use super::Json;

    #[test]
    fn duplicate_fields_and_non_finite_numbers_are_rejected() {
        assert!(
            Json::parse(r#"{"a": 1, "a": 1}"#)
                .unwrap_err()
                .contains("duplicate JSON field")
        );
        assert!(Json::parse(r#"{"a": NaN}"#).is_err());
        assert!(Json::parse("1e999").is_err());
    }

    #[test]
    fn integers_remain_distinct_from_decimals_and_fields_keep_their_order() {
        let value = Json::parse(r#"{"b": 1, "a": 1.0}"#).unwrap();
        assert_eq!(value.field_names().unwrap(), ["b", "a"]);
        assert_eq!(value.get("b"), Some(&Json::Integer(1)));
        assert_eq!(value.get("a"), Some(&Json::Decimal(1.0)));
    }
}
