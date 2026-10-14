use std::time::Duration;

use eframe::egui::Color32;

// Name
pub const APP_NAME: &str = "ctrlbn";

// Menu window
pub const MENU_WIDTH: f32 = 420.0;
pub const MENU_HEIGHT: f32 = 220.0;
pub const MENU_CORNER_RADIUS: u8 = 8;
pub const MENU_MARGIN: i8 = 16;
pub const MENU_SPACING: f32 = 10.0;
// Where the window waits while the menu is closed (outside the screen)
pub const HIDDEN_POSITION_X: f32 = -10000.0;
pub const HIDDEN_POSITION_Y: f32 = -10000.0;

// Colors (dark grey, transparent)
pub const MENU_BACKGROUND: Color32 = Color32::from_rgba_premultiplied(35, 35, 35, 220);
pub const TEXT_COLOR: Color32 = Color32::from_gray(225);
pub const COUNTER_COLOR: Color32 = Color32::from_gray(150);

// Font sizes
pub const MENU_FONT_SIZE: f32 = 16.0;
pub const COUNTER_FONT_SIZE: f32 = 13.0;

// Menu text limits
pub const MENU_MAX_LINES: usize = 6;
pub const MENU_MAX_CHARS: usize = 280;

// History
pub const MAX_HISTORY_ENTRIES: usize = 50;
pub const HISTORY_FILE_NAME: &str = "clipboard_history.txt";

// Terminal
pub const TERMINAL_MAX_CHARS: usize = 60;

// Timing
pub const CLIPBOARD_POLL_INTERVAL: Duration = Duration::from_millis(500);
pub const HOTKEY_POLL_INTERVAL: Duration = Duration::from_millis(20);
pub const HIDDEN_REPAINT_INTERVAL: Duration = Duration::from_millis(100);
