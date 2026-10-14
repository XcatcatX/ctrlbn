use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use arboard::Clipboard;

use crate::logic::history_file;
use crate::logic::history_list::SharedHistory;
use crate::theme;

// The text we saw last in the clipboard. Used to ignore texts we already know.
pub type SharedLastText = Arc<Mutex<String>>;

struct WatcherContext {
    history: SharedHistory,
    last_text: SharedLastText,
    file_path: PathBuf,
    on_change: Box<dyn Fn(&str) + Send>,
}

// Starts a thread that checks the clipboard. on_change gets every new text.
pub fn start(
    history: SharedHistory,
    last_text: SharedLastText,
    file_path: PathBuf,
    on_change: impl Fn(&str) + Send + 'static,
) {
    let context = WatcherContext {
        history,
        last_text,
        file_path,
        on_change: Box::new(on_change),
    };
    thread::spawn(move || run_loop(context));
}

// Puts a text into the clipboard without it counting as a new copy
pub fn put_text_in_clipboard(last_text: &SharedLastText, text: &str) {
    let mut last = last_text.lock().unwrap();
    match Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text)) {
        Ok(()) => *last = text.to_string(),
        Err(error) => eprintln!("Cannot write to clipboard: {error}"),
    }
}

fn run_loop(context: WatcherContext) {
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(error) => {
            eprintln!("Cannot open clipboard: {error}");
            return;
        }
    };
    // Text that is already in the clipboard at start is saved, but not printed
    if let Ok(text) = clipboard.get_text() {
        handle_text(&context, text, false);
    }
    loop {
        // Errors happen when the clipboard has no text (e.g. an image). Ignore them.
        if let Ok(text) = clipboard.get_text() {
            handle_text(&context, text, true);
        }
        thread::sleep(theme::CLIPBOARD_POLL_INTERVAL);
    }
}

fn handle_text(context: &WatcherContext, text: String, print_it: bool) {
    if text.trim().is_empty() || !remember_if_new(&context.last_text, &text) {
        return;
    }
    if print_it {
        (context.on_change)(&text);
    }
    add_to_history(context, text);
}

fn remember_if_new(last_text: &SharedLastText, text: &str) -> bool {
    let mut last = last_text.lock().unwrap();
    if *last == text {
        return false;
    }
    *last = text.to_string();
    true
}

fn add_to_history(context: &WatcherContext, text: String) {
    let mut history = context.history.lock().unwrap();
    history.add(text);
    history_file::save(&context.file_path, history.items());
}
