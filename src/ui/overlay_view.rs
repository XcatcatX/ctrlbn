use eframe::egui;

use crate::theme;
use crate::utils::text_format;

// Draws the menu: one entry, big, with a counter (e.g. 3 / 20)
pub fn show_menu(ctx: &egui::Context, entry: Option<&str>, index: usize, total: usize) {
    egui::CentralPanel::default()
        .frame(menu_frame())
        .show(ctx, |ui| match entry {
            Some(text) => show_entry(ui, text, index, total),
            None => show_empty_message(ui),
        });
}

fn menu_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(theme::MENU_BACKGROUND)
        .corner_radius(egui::CornerRadius::same(theme::MENU_CORNER_RADIUS))
        .inner_margin(egui::Margin::same(theme::MENU_MARGIN))
}

fn show_entry(ui: &mut egui::Ui, text: &str, index: usize, total: usize) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
        ui.label(counter_text(index, total));
    });
    ui.add_space(theme::MENU_SPACING);
    let shortened = text_format::shorten_for_menu(text, theme::MENU_MAX_LINES, theme::MENU_MAX_CHARS);
    ui.label(entry_text(shortened));
}

fn show_empty_message(ui: &mut egui::Ui) {
    ui.label(entry_text("No entries yet.".to_string()));
}

fn counter_text(index: usize, total: usize) -> egui::RichText {
    egui::RichText::new(format!("{} / {}", index + 1, total))
        .size(theme::COUNTER_FONT_SIZE)
        .color(theme::COUNTER_COLOR)
}

fn entry_text(text: String) -> egui::RichText {
    egui::RichText::new(text)
        .size(theme::MENU_FONT_SIZE)
        .color(theme::TEXT_COLOR)
}
