mod editor;
mod enums;
mod tab;
mod utility;

use editor::editor::TextEditorApp;
use eframe::{egui, run_native, NativeOptions};
use egui::Key;
use egui::Modifiers;

use utility::utility::get_next_id;
struct KeyManager {
    keys_pressed: Vec<Key>,
    modifiers_pressed: Vec<Modifiers>,
}

fn main() {
    let win_option = NativeOptions::default();
    let _ = run_native(
        "Reditor", 
        win_option, 
        Box::new(|cc: &eframe::CreationContext<'_>| Box::new(TextEditorApp::new(cc)))
    );
}
