/// Calculates the default avatar index for a given user id.
///
/// The calculation is: `(userId >> 22) % 6`
///
/// If the user uses the legacy username system, it is: `discriminator % 5`
///
///
/// # Link
///
/// The default avatars URLs are `https://cdn.discordapp.com/embed/avatars/INDEX.png`.
///
/// You can use [`crate::Cdn::default_user_avatar`] to get the link.
pub fn calculate_user_default_avatar_index(user_id: u64) -> u8 {
    ((user_id >> 22) % 6) as u8
}
