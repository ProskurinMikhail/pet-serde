use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime<Utc>,
    pub lable: String,
}

mod my_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y/%m/%d %H:%M";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

fn main() {
    let data = StructWithCustomDate {
        timestamp: chrono::offset::Utc::now(),
        lable: "Time now!".to_owned(),
    };
    println!("{}", data.timestamp);
    println!("{}", data.lable);

    let serialized = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", serialized);

    let deserialized: StructWithCustomDate = serde_json::from_str(&serialized).unwrap();
    println!("{}", deserialized.timestamp);
    println!("{}", deserialized.lable);
}