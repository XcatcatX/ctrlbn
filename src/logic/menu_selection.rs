use std::sync::{Arc, Mutex};

pub type SharedMenu = Arc<Mutex<MenuSelection>>;

// State of the menu: shown or hidden, and which entry is selected (0 = newest)
#[derive(Default)]
pub struct MenuSelection {
    visible: bool,
    index: usize,
}

impl MenuSelection {
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.index = 0;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    // Next older entry. After the last one it jumps to the newest.
    pub fn select_older(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        self.index = (self.index + 1) % total;
    }

    // Next newer entry. Before the newest one it jumps to the oldest.
    pub fn select_newer(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        self.index = (self.index + total - 1) % total;
    }
}
