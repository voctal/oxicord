/// Formats a text in `**bold**`.
pub fn bold(content: impl AsRef<str>) -> String {
    format!("**{}**", content.as_ref())
}

/// Formats a text in `*italic*`.
pub fn italic(content: impl AsRef<str>) -> String {
    format!("*{}*", content.as_ref())
}

/// Formats a text in `__underline__`.
pub fn underline(content: impl AsRef<str>) -> String {
    format!("__{}__", content.as_ref())
}

/// Formats a text in `~~strikethrough~~`.
pub fn strikethrough(content: impl AsRef<str>) -> String {
    format!("~~{}~~", content.as_ref())
}

/// Formats a text in `||spoiler||`.
pub fn spoiler(content: impl AsRef<str>) -> String {
    format!("||{}||", content.as_ref())
}

/// Formats a text as inline `` `code` ``.
///
/// If the content contains a `` ` ``, double backticks will be used instead.
pub fn inline_code(content: impl AsRef<str>) -> String {
    let content = content.as_ref();
    if content.contains('`') {
        format!("``{content}``")
    } else {
        format!("`{content}`")
    }
}

/// Formats a text in a fenced code block, optionally with a language for syntax highlighting.
///
/// ```
/// use oxicord_formatters::code_block;
/// assert_eq!(code_block("let x = 1;", Some("rust")), "```rust\nlet x = 1;\n```");
/// assert_eq!(code_block("plain", None), "```\nplain\n```");
/// ```
pub fn code_block(content: impl AsRef<str>, language: Option<&str>) -> String {
    match language {
        Some(lang) => format!("```{lang}\n{}\n```", content.as_ref()),
        None => format!("```\n{}\n```", content.as_ref()),
    }
}

/// Prefixes every line of `content` with `> ` to make a block quote.
pub fn block_quote(content: impl AsRef<str>) -> String {
    content
        .as_ref()
        .lines()
        .map(|line| format!("> {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Prefixes the given single-line content with `> `.
///
/// Use [`block_quote`] for multi-line string.
pub fn quote(content: impl AsRef<str>) -> String {
    format!("> {}", content.as_ref())
}

/// Formats a text with an hyperlink: `[text](url)`.
///
/// Does not work in regular message content.
pub fn hyperlink(text: impl AsRef<str>, url: impl AsRef<str>) -> String {
    format!("[{}]({})", text.as_ref(), url.as_ref())
}

/// Like [`hyperlink`] but adds a title tooltip: `[text](url "title")`.
pub fn hyperlink_with_title(
    text: impl AsRef<str>,
    url: impl AsRef<str>,
    title: impl AsRef<str>,
) -> String {
    format!(
        "[{}]({} \"{}\")",
        text.as_ref(),
        url.as_ref(),
        title.as_ref()
    )
}

/// Formats an URL in `<angle brackets>` to suppress the embed preview.
pub fn hide_link_embed(url: impl AsRef<str>) -> String {
    format!("<{}>", url.as_ref())
}

/// A level-1 through level-3 markdown heading (`#`, `##`, `###`).
///
/// `level` is clamped to `1..=3`, which are the only supported heading depth.
pub fn heading(content: impl AsRef<str>, level: u8) -> String {
    let hashes = "#".repeat(level.clamp(1, 3) as usize);
    format!("{hashes} {}", content.as_ref())
}

/// Subtext line (`-# text`).
pub fn subtext(content: impl AsRef<str>) -> String {
    format!("-# {}", content.as_ref())
}

/// Renders an unordered list from the given items.
pub fn unordered_list(items: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    items
        .into_iter()
        .map(|item| format!("- {}", item.as_ref()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Renders an ordered (with numbers) list from the given items, starting at 1.
pub fn ordered_list(items: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    items
        .into_iter()
        .enumerate()
        .map(|(i, item)| format!("{}. {}", i + 1, item.as_ref()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Escapes markdown special characters in `content`.
pub fn escape_markdown(content: impl AsRef<str>) -> String {
    let mut out = String::with_capacity(content.as_ref().len());
    for ch in content.as_ref().chars() {
        if matches!(
            ch,
            '*' | '_' | '~' | '`' | '|' | '>' | '#' | '-' | '(' | ')' | '[' | ']' | '\\'
        ) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}
