use serde::{Deserialize, Deserializer, Serializer};
use time::OffsetDateTime;

/// Format string for datetime serialization/deserialization: 2025-03-09T11:26:05.000Z
const DATETIME_FORMAT: &str = "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z";

/// Serializes an OffsetDateTime to an ISO 8601 / RFC 3339 formatted string in format 2025-03-09T11:26:05.000Z
pub fn serialize_datetime<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format to 2025-03-09T11:26:05.000Z (with 3 digit milliseconds)

    // Create a custom format that includes subseconds with 3 digits
    let format =
        time::format_description::parse(DATETIME_FORMAT).map_err(serde::ser::Error::custom)?;

    let formatted = datetime
        .format(&format)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&formatted)
}

/// Deserializes an ISO 8601 / RFC 3339 formatted string to an OffsetDateTime
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Use the same format as serialization to ensure consistency
    let format =
        time::format_description::parse(DATETIME_FORMAT).map_err(serde::de::Error::custom)?;

    OffsetDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
}
