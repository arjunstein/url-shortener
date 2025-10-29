use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// Serialize required datetime
pub fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = dt.with_timezone(&Local).format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

// Serialize optional datetime
pub fn serialize_option_datetime<S>(
    dt: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(v) => serializer.serialize_str(&v.with_timezone(&Local).format(FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

// Deserialize optional datetime (request payload)
pub fn deserialize_option_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let naive =
                NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
            let local_dt = Local
                .from_local_datetime(&naive)
                .earliest()
                .ok_or_else(|| serde::de::Error::custom("invalid datetime"))?;
            Ok(Some(local_dt.with_timezone(&Utc)))
        }
        None => Ok(None),
    }
}
