use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::logic::clipboard_watcher::{self, SharedLastText};
use crate::logic::history_file;
use crate::logic::history_list::{HistoryList, SharedHistory};
use crate::logic::hotkey_watcher;
use crate::logic::menu_selection::{MenuSelection, SharedMenu};
use crate::theme;
use crate::ui::{overlay_view, terminal_view};

pub struct ClipboardApp {
    history: SharedHistory,
    menu: SharedMenu,
    // None = unknown (first frame), so the window state is always set once
    window_shown: Option<bool>,
}

impl ClipboardApp {
    pub fn new(ctx: &egui::Context) -> Self {
        let file_path = history_file::default_path();
        let history = load_history(&file_path);
        let menu: SharedMenu = Arc::new(Mutex::new(MenuSelection::default()));
        let last_text: SharedLastText = Arc::new(Mutex::new(String::new()));

        start_clipboard_watcher(&history, &last_text, file_path);
        start_hotkey_watcher(ctx, &history, &menu, &last_text);

        Self {
            history,
            menu,
            window_shown: None,
        }
    }

    fn read_menu_state(&self) -> (bool, usize) {
        let menu = self.menu.lock().unwrap();
        (menu.is_visible(), menu.index())
    }

    // The window is never hidden. It is moved off the screen and back, so it keeps running.
    fn apply_window_position(&mut self, ctx: &egui::Context, visible: bool) {
        if self.window_shown == Some(visible) {
            return;
        }
        let position = if visible {
            centered_position(ctx)
        } else {
            egui::Pos2::new(theme::HIDDEN_POSITION_X, theme::HIDDEN_POSITION_Y)
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
        ctx.request_repaint();
        self.window_shown = Some(visible);
    }

    fn draw_menu(&self, ctx: &egui::Context, index: usize) {
        let history = self.history.lock().unwrap();
        let entry = history.get(index).map(String::as_str);
        overlay_view::show_menu(ctx, entry, index, history.len());
    }
}

impl eframe::App for ClipboardApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (visible, index) = self.read_menu_state();
        self.apply_window_position(ctx, visible);
        if visible {
            self.draw_menu(ctx, index);
        } else {
            // Safety net so the window keeps running
            ctx.request_repaint_after(theme::HIDDEN_REPAINT_INTERVAL);
        }
    }
}

pub fn viewport_builder() -> egui::ViewportBuilder {
    egui::ViewportBuilder::default()
        .with_title(theme::APP_NAME)
        .with_inner_size([theme::MENU_WIDTH, theme::MENU_HEIGHT])
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top()
        .with_resizable(false)
        .with_taskbar(false)
        .with_position([theme::HIDDEN_POSITION_X, theme::HIDDEN_POSITION_Y])
}

fn load_history(file_path: &PathBuf) -> SharedHistory {
    let items = history_file::load(file_path);
    Arc::new(Mutex::new(HistoryList::from_items(items)))
}

fn start_clipboard_watcher(history: &SharedHistory, last_text: &SharedLastText, file_path: PathBuf) {
    clipboard_watcher::start(
        Arc::clone(history),
        Arc::clone(last_text),
        file_path,
        terminal_view::print_new_entry,
    );
}

fn start_hotkey_watcher(
    ctx: &egui::Context,
    history: &SharedHistory,
    menu: &SharedMenu,
    last_text: &SharedLastText,
) {
    let ctx_for_wake = ctx.clone();
    hotkey_watcher::start(
        Arc::clone(history),
        Arc::clone(menu),
        Arc::clone(last_text),
        move || ctx_for_wake.request_repaint(),
    );
}

fn centered_position(ctx: &egui::Context) -> egui::Pos2 {
    let monitor_size = ctx
        .input(|input| input.viewport().monitor_size)
        .unwrap_or(egui::Vec2::new(1920.0, 1080.0));
    egui::Pos2::new(
        (monitor_size.x - theme::MENU_WIDTH) / 2.0,
        (monitor_size.y - theme::MENU_HEIGHT) / 2.0,
    )
}
