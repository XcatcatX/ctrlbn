// One line, extra spaces and line breaks removed, cut after max_chars
pub fn one_line_preview(text: &str, max_chars: usize) -> String {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let (shortened, was_cut) = cut_chars(&single_line, max_chars);
    add_dots_if_cut(shortened, was_cut)
}

// Keeps line breaks, but only the first lines and the first characters
pub fn shorten_for_menu(text: &str, max_lines: usize, max_chars: usize) -> String {
    let mut lines = text.lines();
    let first_lines: Vec<&str> = lines.by_ref().take(max_lines).collect();
    let lines_were_cut = lines.next().is_some();
    let (shortened, chars_were_cut) = cut_chars(&first_lines.join("\n"), max_chars);
    add_dots_if_cut(shortened, lines_were_cut || chars_were_cut)
}

fn cut_chars(text: &str, max_chars: usize) -> (String, bool) {
    let mut chars = text.chars();
    let shortened: String = chars.by_ref().take(max_chars).collect();
    let was_cut = chars.next().is_some();
    (shortened, was_cut)
}

fn add_dots_if_cut(mut text: String, was_cut: bool) -> String {
    if was_cut {
        text.push_str("...");
    }
    text
}
