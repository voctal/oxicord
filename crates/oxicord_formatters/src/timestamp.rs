//! Unix timestamp formatters `<t:unix:style>`

use std::fmt;

/// Display style for a Discord dynamic timestamp tag.
///
/// See <https://discord.com/developers/docs/reference#message-formatting-timestamp-styles>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimestampStyle {
    /// `16:20` (style `t`)
    ShortTime,
    /// `16:20:30` (style `T`)
    MediumTime,
    /// `20/04/2021` (style `d`)
    ShortDate,
    /// `April 20, 2021` (style `D`)
    LongDate,
    /// `April 20, 2021 at 16:20` (style `f`), the default style.
    #[default]
    LongDateShortTime,
    /// `Tuesday, April 20, 2021 at 16:20` (style `F`)
    FullDateShortTime,
    /// `20/04/2021, 16:20` (style `s`)
    ShortDateShortTime,
    /// `20/04/2021, 16:20:30` (style `S`)
    ShortDateMediumTime,
    /// `4 years ago` (style `R`)
    RelativeTime,
}

impl TimestampStyle {
    fn as_char(self) -> char {
        match self {
            TimestampStyle::ShortTime => 't',
            TimestampStyle::MediumTime => 'T',
            TimestampStyle::ShortDate => 'd',
            TimestampStyle::LongDate => 'D',
            TimestampStyle::LongDateShortTime => 'f',
            TimestampStyle::FullDateShortTime => 'F',
            TimestampStyle::ShortDateShortTime => 's',
            TimestampStyle::ShortDateMediumTime => 'S',
            TimestampStyle::RelativeTime => 'R',
        }
    }
}

impl fmt::Display for TimestampStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

/// Formats a Discord timestamp from a Unix timestamp (seconds), e.g.
/// `<t:1618922400:R>`, to display the given timestamp in the user's timezone and locale.
///
/// ```
/// use oxicord_formatters::{time, TimestampStyle};
/// assert_eq!(time(1618922400, TimestampStyle::RelativeTime), "<t:1618922400:R>");
/// assert_eq!(time(1618922400, TimestampStyle::default()), "<t:1618922400:f>");
/// ```
pub fn time(seconds: i64, style: TimestampStyle) -> String {
    format!("<t:{seconds}:{style}>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_formatter() {
        assert_eq!(
            time(1618922400, TimestampStyle::RelativeTime),
            "<t:1618922400:R>"
        );
        assert_eq!(
            time(1618922400, TimestampStyle::default()),
            "<t:1618922400:f>"
        );
    }
}
