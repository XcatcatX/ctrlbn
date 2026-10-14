use crate::theme;
use crate::utils::text_format;

// Prints one new copy as a single line, without a number
pub fn print_new_entry(text: &str) {
    println!("{}", text_format::one_line_preview(text, theme::TERMINAL_MAX_CHARS));
}
