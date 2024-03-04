use super::CiderVersion;

use serde::ser::{Serialize, Serializer};

impl Serialize for CiderVersion {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer {
		let formatted = format!(
			"{}.{}.{}.{}{}", 
			self.edition,
			self.major,
			self.minor,
			self.patch,

			if !self.extra.is_empty() {
				format!("-{}", self.extra)
			} else {
				"".to_string()
			}
		);

		serializer.serialize_str(&formatted)
	}
}