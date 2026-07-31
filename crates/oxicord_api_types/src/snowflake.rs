//! Discord ID type.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use std::fmt;
use std::str::FromStr;

/// Discord epoch in ms, which is 2015-01-01T00:00:00.000Z.
pub const DISCORD_EPOCH: u64 = 1_420_070_400_000;

/// https://docs.discord.com/developers/reference#snowflakes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Snowflake(pub u64);

impl Snowflake {
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The timestamp encoded in this id, in milliseconds.
    #[inline]
    pub const fn timestamp(self) -> u64 {
        (self.0 >> 22) + DISCORD_EPOCH
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Snowflake {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl From<u64> for Snowflake {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<Snowflake> for u64 {
    fn from(v: Snowflake) -> Self {
        v.0
    }
}

impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // The API only takes snowflakes as strings
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Accepts int or str, and deserialize in Snowflake

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StrOrInt {
            Str(String),
            Int(u64),
        }
        match StrOrInt::deserialize(deserializer)? {
            StrOrInt::Str(s) => s.parse().map(Snowflake).map_err(D::Error::custom),
            StrOrInt::Int(i) => Ok(Snowflake(i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_string() {
        let json = serde_json::to_string(&Snowflake(123456789012345678)).unwrap();
        assert_eq!(json, r#""123456789012345678""#);
    }

    #[test]
    fn deserializes_from_string() {
        let sf: Snowflake = serde_json::from_str(r#""123456789012345678""#).unwrap();
        assert_eq!(sf.get(), 123456789012345678);
    }

    #[test]
    fn deserializes_from_number() {
        let sf: Snowflake = serde_json::from_str("123456789012345678").unwrap();
        assert_eq!(sf.get(), 123456789012345678);
    }
}
