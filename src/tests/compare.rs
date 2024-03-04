use crate::CiderVersion;

#[test]
fn cider_versions() {
	let lhs = CiderVersion::new(2u64, 3u64, 4u64, 5u64, "RC0");
	let rhs = CiderVersion::new(2u64, 4u64, 4u64, 5u64, "RC0");

	assert!(lhs < rhs);
}

#[test]
fn cider_versions_2() {
	let lhs = CiderVersion::new(3u64, 3u64, 4u64, 5u64, "RC0");
	let rhs = CiderVersion::new(2u64, 4u64, 4u64, 5u64, "RC0");

	assert!(lhs > rhs);
}


#[test]
fn cider_versions_prerelease() {
	let lhs = CiderVersion::basic(2u64, 3u64, 4u64, 6u64);
	let rhs = CiderVersion::new(2u64, 3u64, 4u64, 5u64, "RC0");

	assert!(lhs > rhs);
}