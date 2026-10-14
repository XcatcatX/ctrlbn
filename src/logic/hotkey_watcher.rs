use std::thread;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::logic::clipboard_watcher::{self, SharedLastText};
use crate::logic::history_list::SharedHistory;
use crate::logic::menu_selection::SharedMenu;
use crate::theme;

struct HotkeyContext {
    history: SharedHistory,
    menu: SharedMenu,
    last_text: SharedLastText,
    wake_ui: Box<dyn Fn() + Send>,
}

#[derive(Default, Clone, Copy)]
struct KeyState {
    ctrl: bool,
    key_b: bool,
    key_n: bool,
}

impl KeyState {
    fn from_keys(keys: &[Keycode]) -> Self {
        Self {
            ctrl: keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl),
            key_b: keys.contains(&Keycode::B),
            key_n: keys.contains(&Keycode::N),
        }
    }
}

// Starts a thread that watches Ctrl+B, Ctrl+N and the release of Ctrl.
// wake_ui is called whenever the menu changed.
pub fn start(
    history: SharedHistory,
    menu: SharedMenu,
    last_text: SharedLastText,
    wake_ui: impl Fn() + Send + 'static,
) {
    let context = HotkeyContext {
        history,
        menu,
        last_text,
        wake_ui: Box::new(wake_ui),
    };
    thread::spawn(move || run_loop(context));
}

fn run_loop(context: HotkeyContext) {
    let device_state = DeviceState::new();
    let mut previous = KeyState::default();
    loop {
        let current = KeyState::from_keys(&device_state.get_keys());
        handle_key_change(&context, previous, current);
        previous = current;
        thread::sleep(theme::HOTKEY_POLL_INTERVAL);
    }
}

// Ctrl+N = next older entry, Ctrl+B = next newer entry
fn handle_key_change(context: &HotkeyContext, previous: KeyState, current: KeyState) {
    let n_pressed = current.key_n && !previous.key_n;
    let b_pressed = current.key_b && !previous.key_b;
    let ctrl_released = previous.ctrl && !current.ctrl;

    if current.ctrl && n_pressed {
        move_menu(context, true);
    } else if current.ctrl && b_pressed {
        move_menu(context, false);
    } else if ctrl_released {
        close_menu_and_copy(context);
    }
}

// First press opens the menu at the newest entry. Next presses move the selection.
fn move_menu(context: &HotkeyContext, to_older: bool) {
    let total = context.history.lock().unwrap().len();
    {
        let mut menu = context.menu.lock().unwrap();
        if !menu.is_visible() {
            menu.open();
        } else if to_older {
            menu.select_older(total);
        } else {
            menu.select_newer(total);
        }
    }
    (context.wake_ui)();
}

// Hides the menu and puts the selected text into the clipboard
fn close_menu_and_copy(context: &HotkeyContext) {
    let (was_visible, index) = {
        let mut menu = context.menu.lock().unwrap();
        let state = (menu.is_visible(), menu.index());
        menu.close();
        state
    };
    if !was_visible {
        return;
    }
    let selected_text = context.history.lock().unwrap().get(index).cloned();
    if let Some(text) = selected_text {
        clipboard_watcher::put_text_in_clipboard(&context.last_text, &text);
    }
    (context.wake_ui)();
}
