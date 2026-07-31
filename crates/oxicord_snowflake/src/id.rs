use std::{fmt, marker::PhantomData, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

use crate::{extract_timestamp, timestamp_to_snowflake};

/// A Discord Snowflake ID.
///
/// This is a simple (zero-cost) wrapper around u64.
#[repr(transparent)]
pub struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    /// Creates a new ID from its raw snowflake value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self::from_raw(value)
    }

    /// Creates a new ID from its raw snowflake value.
    #[inline]
    pub const fn from_raw(value: u64) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Returns the raw snowflake value.
    #[inline]
    pub const fn get(self) -> u64 {
        self.value
    }

    /// Casts this ID into another ID type.
    ///
    /// This is a compile-time type conversion only.
    #[inline]
    pub const fn cast<U>(self) -> Id<U> {
        Id::from_raw(self.value)
    }

    /// Creates the smallest snowflake for the given timestamp.
    ///
    /// This is useful when querying messages before/after a timestamp.
    ///
    /// # Panics
    ///
    /// Panics if the timestamp is before the Discord epoch.
    #[inline]
    pub const fn from_timestamp(timestamp: u64) -> Self {
        Self::from_raw(timestamp_to_snowflake(timestamp))
    }

    /// Returns the timestamp in milliseconds.
    #[inline]
    pub const fn timestamp(self) -> u64 {
        extract_timestamp(self.value)
    }

    /// Returns the internal worker id.
    #[inline]
    pub const fn worker_id(self) -> u8 {
        ((self.value >> 17) & 0x1F) as u8
    }

    /// Returns the internal process id.
    #[inline]
    pub const fn process_id(self) -> u8 {
        ((self.value >> 12) & 0x1F) as u8
    }

    /// Returns the increment.
    #[inline]
    pub const fn increment(self) -> u16 {
        (self.value & 0xFFF) as u16
    }
}

// Note:
// The following impl need to be done manually
// since using #[derive] will not impl for T,
// meaning the markers wont have the impls.

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Id<T> {
    #[inline]
    fn default() -> Self {
        Self::from_raw(0)
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.value).finish()
    }
}

impl<T> PartialEq for Id<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for Id<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> std::hash::Hash for Id<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> fmt::Display for Id<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> From<Id<T>> for u64 {
    #[inline]
    fn from(value: Id<T>) -> Self {
        value.value
    }
}

impl<T> FromStr for Id<T> {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_raw(s.parse()?))
    }
}

// serde impl

impl<T> Serialize for Id<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // The API only takes snowflakes as strings
        serializer.serialize_str(&self.get().to_string())
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Accepts int or str, and deserialize in Snowflake

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StrOrInt {
            Str(String),
            Int(u64),
        }
        match StrOrInt::deserialize(deserializer)? {
            StrOrInt::Str(s) => s.parse().map(Id::from_raw).map_err(D::Error::custom),
            StrOrInt::Int(i) => Ok(Id::from_raw(i)),
        }
    }
}

// size validation

const _: () = {
    assert!(std::mem::size_of::<Id<()>>() == std::mem::size_of::<u64>());
};

#[cfg(test)]
mod tests {
    use crate::{ChannelId, DISCORD_EPOCH, MessageId, UserId};

    const RAW: u64 = 175_928_847_299_117_063;

    // serde tests

    #[test]
    fn serializes_as_string() {
        let json = serde_json::to_string(&UserId::from_raw(123456789012345678)).unwrap();
        assert_eq!(json, r#""123456789012345678""#);
    }

    #[test]
    fn deserializes_from_string() {
        let sf: UserId = serde_json::from_str(r#""123456789012345678""#).unwrap();
        assert_eq!(sf.get(), 123456789012345678);
    }

    #[test]
    fn deserializes_from_number() {
        let sf: UserId = serde_json::from_str("123456789012345678").unwrap();
        assert_eq!(sf.get(), 123456789012345678);
    }

    // other tests

    #[test]
    fn size_is_zero_cost() {
        assert_eq!(std::mem::size_of::<UserId>(), 8);
        assert_eq!(std::mem::size_of::<ChannelId>(), 8);
        assert_eq!(std::mem::size_of::<MessageId>(), 8);
        assert_eq!(std::mem::size_of::<UserId>(), std::mem::size_of::<u64>());
    }

    #[test]
    fn new_and_get() {
        let id = UserId::new(RAW);

        assert_eq!(id.get(), RAW);
    }

    #[test]
    fn from_raw() {
        let id = UserId::from_raw(RAW);

        assert_eq!(id.get(), RAW);
    }

    #[test]
    fn into_u64() {
        let id = UserId::new(RAW);

        let raw: u64 = id.into();

        assert_eq!(raw, RAW);
    }

    #[test]
    fn display() {
        let id = UserId::new(RAW);

        assert_eq!(id.to_string(), RAW.to_string());
    }

    #[test]
    fn from_str() {
        let id: UserId = RAW.to_string().parse().unwrap();

        assert_eq!(id.get(), RAW);
    }

    #[test]
    fn default_is_zero() {
        let id = UserId::default();

        assert_eq!(id.get(), 0);
    }

    #[test]
    fn cast_is_zero_cost() {
        let user_id = UserId::new(RAW);

        let channel_id: ChannelId = user_id.cast();

        assert_eq!(channel_id.get(), RAW);
    }

    #[test]
    fn cast_keeps_same_memory_representation() {
        let user_id = UserId::new(RAW);
        let message_id: MessageId = user_id.cast();

        assert_eq!(user_id.get(), message_id.get());
    }

    #[test]
    fn timestamp_conversion() {
        let timestamp = 1_700_000_000_000;

        let id = MessageId::from_timestamp(timestamp);

        assert_eq!(id.timestamp(), timestamp);
    }

    #[test]
    #[should_panic]
    fn timestamp_before_discord_epoch_panics() {
        MessageId::from_timestamp(DISCORD_EPOCH - 1);
    }

    #[test]
    fn data_extraction() {
        let timestamp_part = 100u64 << 22;
        let worker_part = 7u64 << 17;
        let increment_part = 12u64;
        let process_part = 19u64 << 12;

        let id = UserId::from_raw(timestamp_part | worker_part | increment_part | process_part);

        assert_eq!(id.timestamp(), 100 + DISCORD_EPOCH);
        assert_eq!(id.worker_id(), 7);
        assert_eq!(id.increment(), 12);
        assert_eq!(id.process_id(), 19);
    }

    #[test]
    fn process_id_extraction() {
        let timestamp_part = 100u64 << 22;
        let process_part = 13u64 << 12;

        let id = UserId::from_raw(timestamp_part | process_part);

        assert_eq!(id.process_id(), 13);
    }

    #[test]
    fn increment_extraction() {
        let timestamp_part = 100u64 << 22;
        let increment_part = 4095u64;

        let id = UserId::from_raw(timestamp_part | increment_part);

        assert_eq!(id.increment(), 4095);
    }

    #[test]
    fn ordering_works() {
        let a = UserId::new(1);
        let b = UserId::new(2);

        assert!(a < b);
    }

    #[test]
    fn different_ids_are_not_equal() {
        let a = UserId::new(1);
        let b = UserId::new(2);

        assert_ne!(a, b);
    }

    #[test]
    fn marker_types_do_not_affect_value() {
        let user: UserId = UserId::new(RAW);
        let channel: ChannelId = ChannelId::new(RAW);

        assert_eq!(user.get(), channel.get());
    }

    #[test]
    fn invalid_parse_fails() {
        let result: Result<UserId, _> = "not-a-number".parse();

        assert!(result.is_err());
    }
}
