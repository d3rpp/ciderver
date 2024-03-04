use super::CiderVersion;

use regex::{Match, Regex};
use serde::{
    de::{Error, Visitor},
    Deserialize,
};

struct CiderVersionVisitor;

const CIDER_VERSION_REGEX_STR: &str = r#"^(?P<edition>0|[1-9]\d*)\.(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<extra>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?$"#;
const SEMVER_REGEX_STR: &str = r#"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#;

lazy_static! {
    static ref CIDER_VERSION_REGEX: Regex = Regex::new(CIDER_VERSION_REGEX_STR).unwrap();
    static ref SEMVER_REGEX: Regex = Regex::new(SEMVER_REGEX_STR).unwrap();
}

fn parse_int<E>(inp: Option<Match<'_>>) -> Result<u64, E> where E: Error {
	if let Some(mat) = inp {
		mat.as_str().parse::<u64>().map_err(|e| Error::custom(e.to_string()))
	} else {
		Err(Error::custom("Match is None, the regex is broken"))
	}
}

fn parse_string<E>(inp: Option<Match<'_>>) -> Result<String, E> where E: Error {
	if let Some(mat) = inp {
		Ok(String::from(mat.as_str()))
	} else {
		Err(Error::custom("Match is None, the regex is broken"))
	}
}

impl<'de> Visitor<'de> for CiderVersionVisitor {
    type Value = CiderVersion;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a cider-flavoured semver (e.g. 2.3.2.0)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(captures) = CIDER_VERSION_REGEX.captures(v) {
            Ok(Self::Value {
                edition: parse_int(captures.name("edition"))?,
                major: parse_int(captures.name("major"))?,
                minor: parse_int(captures.name("minor"))?,
                patch: parse_int(captures.name("patch"))?,
                extra: parse_string::<E>(captures.name("extra")).unwrap_or_default(),
            })
        } else if SEMVER_REGEX.is_match(v) {
            let semver = semver::Version::parse(v).map_err(|e| Error::custom(e))?;

            Ok(Self::Value {
                edition: semver.major,
                major: semver.minor,
                minor: semver.patch,
                patch: 0,
                extra: semver.pre.to_string(),
            })
        } else {
            Err(Error::custom("CIDER_VERSION_REGEX failed to parse version"))
        }
    }
}

impl<'de> Deserialize<'de> for CiderVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = CiderVersionVisitor;

        deserializer.deserialize_str(visitor)
    }
}
