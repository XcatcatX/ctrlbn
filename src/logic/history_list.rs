use std::sync::{Arc, Mutex};

use crate::theme;

pub type SharedHistory = Arc<Mutex<HistoryList>>;

// List of copied texts, newest first
pub struct HistoryList {
    items: Vec<String>,
}

impl HistoryList {
    pub fn from_items(mut items: Vec<String>) -> Self {
        items.truncate(theme::MAX_HISTORY_ENTRIES);
        Self { items }
    }

    // New text goes to the front. An equal old text is removed first.
    pub fn add(&mut self, text: String) {
        self.items.retain(|item| *item != text);
        self.items.insert(0, text);
        self.items.truncate(theme::MAX_HISTORY_ENTRIES);
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
