use serde_test::{*, Token::*};

use crate::CiderVersion;

#[test]
fn test_valid_semver_string() {
	let basic_semver = CiderVersion::new(2u64, 3u64, 4u64, 0u64, "RC0");

	assert_de_tokens(
		&basic_semver, 
		&[
			Str("2.3.4-RC0")
		]
	);
}

#[test]
fn test_valid_ciderver_string() {
	let basic_ciderver = CiderVersion::new(2u64, 3u64, 4u64, 5u64, "RC0");

	assert_de_tokens(
		&basic_ciderver, 
		&[
			Str("2.3.4.5-RC0")
		]
	);
}