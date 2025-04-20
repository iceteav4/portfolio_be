use serde::{Deserialize, Deserializer, Serializer};
use time::OffsetDateTime;

/// Serializes an OffsetDateTime to an ISO 8601 / RFC 3339 formatted string in format 2025-03-09T11:26:05Z
pub fn serialize_datetime<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format to 2025-03-09T11:26:05Z (without milliseconds)
    
    // Create a custom format that excludes subseconds
    let format = time::format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
    ).map_err(serde::ser::Error::custom)?;
    
    let formatted = datetime.format(&format).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&formatted)
}

/// Deserializes an ISO 8601 / RFC 3339 formatted string to an OffsetDateTime
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        .map_err(serde::de::Error::custom)
}
