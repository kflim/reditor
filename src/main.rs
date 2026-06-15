mod editor;
mod enums;
mod tab;
mod utility;

use editor::editor::TextEditorApp;
use eframe::{egui, run_native, NativeOptions};
use egui::Vec2;
use egui::ViewportBuilder;

fn main() {
    let win_option = NativeOptions {
        viewport: ViewportBuilder::default().with_min_inner_size(Vec2::new(700.0, 400.0)),
        ..Default::default()
    };

    let _ = run_native(
        "Reditor",
        win_option,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(TextEditorApp::new(cc)))),
    );
}
