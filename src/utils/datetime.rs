use serde::{Deserialize, Deserializer, Serializer};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

/// Serializes an OffsetDateTime to an ISO 8601 / RFC 3339 formatted string in format 2025-03-09T11:26:05.000Z
pub fn serialize_datetime<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Use RFC 3339 format which properly handles timezone information
    let formatted = datetime
        .format(&Rfc3339)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&formatted)
}

/// Deserializes an ISO 8601 / RFC 3339 formatted string to an OffsetDateTime
#[allow(unused)]
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Use RFC 3339 format which properly handles timezone information
    OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)
}

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            // Use RFC 3339 format which properly handles timezone information
            Ok(Some(
                OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?,
            ))
        }
        None => Ok(None),
    }
}
