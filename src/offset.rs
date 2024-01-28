use chrono::Duration;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// The following has been generated with the help of ChatGPT

#[derive(Debug)]
pub struct Offset(Duration);

impl Serialize for Offset {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let total_seconds = self.0.num_seconds();
		let hours = total_seconds / 3600;
		let minutes = (total_seconds % 3600) / 60;
		let seconds = total_seconds % 60;
		let formatted_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
		serializer.serialize_str(&formatted_time)
	}
}

impl<'de> Deserialize<'de> for Offset {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct DurationVisitor;

		impl<'de> Visitor<'de> for DurationVisitor {
			type Value = Offset;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a string in the format HH:MM:SS")
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				let parts: Vec<&str> = value.split(':').collect();
				if parts.len() != 3 {
					return Err(E::custom("Time should be in the format HH:MM:SS"));
				}
				let hours: i64 = parts[0].parse().map_err(E::custom)?;
				let minutes: i64 = parts[1].parse().map_err(E::custom)?;
				let seconds: i64 = parts[2].parse().map_err(E::custom)?;
				Ok(Offset(Duration::seconds(
					hours * 3600 + minutes * 60 + seconds,
				)))
			}
		}

		deserializer.deserialize_str(DurationVisitor)
	}
}
