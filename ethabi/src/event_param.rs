//! Event param specification.

use {ParamType};
use rstd::prelude::*;
use rstd::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Event param specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct EventParam {
	/// Param name.
	pub name: String,
	/// Param type.
	#[cfg_attr(feature = "std", serde(rename="type"))]
	pub kind: ParamType,
	/// Indexed flag. If true, param is used to build block bloom.
	pub indexed: bool,
}

#[cfg(test)]
mod tests {
	use serde_json;
	use {EventParam, ParamType};

	#[test]
	fn event_param_deserialization() {
		let s = r#"{
			"name": "foo",
			"type": "address",
			"indexed": true
		}"#;

		let deserialized: EventParam = serde_json::from_str(s).unwrap();

		assert_eq!(deserialized, EventParam {
			name: "foo".to_owned(),
			kind: ParamType::Address,
			indexed: true,
		});
	}
}
