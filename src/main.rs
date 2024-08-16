mod editor;
mod enums;
mod tab;
mod utility;

use editor::editor::TextEditorApp;
use eframe::{egui, run_native, NativeOptions};

use egui::Vec2;
use egui::ViewportBuilder;
use utility::utility::get_next_id;

fn main() {
    let win_option = NativeOptions {
        viewport: ViewportBuilder::default().with_min_inner_size(Vec2::new(700.0, 400.0)),
        vsync: NativeOptions::default().vsync,
        multisampling: NativeOptions::default().multisampling,
        depth_buffer: NativeOptions::default().depth_buffer,
        hardware_acceleration: NativeOptions::default().hardware_acceleration,
        stencil_buffer: NativeOptions::default().stencil_buffer,
        renderer: NativeOptions::default().renderer,
        follow_system_theme: NativeOptions::default().follow_system_theme,
        run_and_return: NativeOptions::default().run_and_return,
        event_loop_builder: NativeOptions::default().event_loop_builder,
        default_theme: NativeOptions::default().default_theme,
        window_builder: NativeOptions::default().window_builder,
        shader_version: NativeOptions::default().shader_version,
        centered: NativeOptions::default().centered,
        persist_window: NativeOptions::default().persist_window,
        persistence_path: NativeOptions::default().persistence_path,
    };

    let _ = run_native(
        "Reditor",
        win_option,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(TextEditorApp::new(cc)))),
    );
}
