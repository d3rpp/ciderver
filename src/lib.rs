#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;
use std::fmt::Display;

mod de;
mod ser;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CiderVersion {
    edition: u64,
    major: u64,
    minor: u64,
    patch: u64,
    extra: String,
}

impl PartialOrd for CiderVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.edition.partial_cmp(&other.edition) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.major.partial_cmp(&other.major) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.minor.partial_cmp(&other.minor) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.patch.partial_cmp(&other.patch) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        // empty extras are greater than full extras
        match (self.extra.trim().is_empty(), other.extra.trim().is_empty()) {
            (false, true) => Some(Ordering::Less),
            (true, false) => Some(Ordering::Greater),
            (true, true) => Some(Ordering::Equal),
            (false, false) => self.extra.partial_cmp(&other.extra)
        }

    }
}

impl CiderVersion {
    #[must_use]
    pub fn new(
        edition: impl Into<u64>,
        major: impl Into<u64>,
        minor: impl Into<u64>,
        patch: impl Into<u64>,
        extra: impl Display,
    ) -> Self {
        Self {
            edition: edition.into(),
            major: major.into(),
            minor: minor.into(),
            patch: patch.into(),
            extra: extra.to_string(),
        }
    }

    #[must_use]
    pub fn basic(
        edition: impl Into<u64>,
        major: impl Into<u64>,
        minor: impl Into<u64>,
        patch: impl Into<u64>,
    ) -> Self {
        Self::new(edition, major, minor, patch, String::default())
    }

    #[must_use]
    pub fn edition(&self) -> u64 {
        self.edition
    }

    #[must_use]
    pub fn major(&self) -> u64 {
        self.major
    }

    #[must_use]
    pub fn minor(&self) -> u64 {
        self.minor
    }

    #[must_use]
    pub fn patch(&self) -> u64 {
        self.patch
    }

    #[must_use]
    pub fn extra(&self) -> &str {
        &self.extra
    }

    pub fn set_edition(&mut self, edition: impl Into<u64>) {
        self.edition = edition.into();
    }

    pub fn set_major(&mut self, major: impl Into<u64>) {
        self.major = major.into();
    }

    pub fn set_minor(&mut self, minor: impl Into<u64>) {
        self.minor = minor.into();
    }

    pub fn set_patch(&mut self, patch: impl Into<u64>) {
        self.patch = patch.into();
    }

    pub fn set_extra(&mut self, extra: impl Display) {
        self.extra = extra.to_string();
    }
}
