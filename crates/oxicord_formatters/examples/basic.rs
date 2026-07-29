use oxicord_formatters::*;

fn main() {
    println!("{}", bold("Welcome!"));
    println!("{}", user_mention(123456789012345678));
    println!("{}", code_block("fn main() {}", Some("rust")));
    println!("{}", time(1_718_000_000, TimestampStyle::RelativeTime));
    println!(
        "{}",
        hyperlink("oxicord", "https://github.com/voctal/oxicord")
    );
    println!(
        "{}\n{}",
        heading("List title", 2),
        unordered_list(["Something", "Something else", "Something else again"])
    );
}
