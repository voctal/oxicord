/// Discord epoch (2015-01-01T00:00:00Z) in milliseconds.
pub const DISCORD_EPOCH: u64 = 1_420_070_400_000;

/// Converts a timestamp (milliseconds) into the smallest
/// Discord snowflake for that timestamp.
///
/// This is useful for pagination.
///
/// # Panics
///
/// Panics if `timestamp` is before the Discord epoch.
#[inline]
pub const fn timestamp_to_snowflake(timestamp: u64) -> u64 {
    assert!(timestamp >= DISCORD_EPOCH);

    (timestamp - DISCORD_EPOCH) << 22
}

/// Extracts the timestamp (milliseconds) from a Discord snowflake.
#[inline]
pub const fn extract_timestamp(raw: u64) -> u64 {
    (raw >> 22) + DISCORD_EPOCH
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_timestamp_conversion() {
        let timestamp = 1_700_000_000_000;

        let raw = timestamp_to_snowflake(timestamp);

        assert_eq!(extract_timestamp(raw), timestamp);
    }
}
