pub mod editor {
    use eframe::App;
    use egui::CentralPanel;
    use egui::Color32;
    use egui::Key;
    use egui::TopBottomPanel;
    use egui_dock::DockArea;
    use egui_dock::DockState;
    use egui_dock::Style;

    use crate::tab::tab::MyTabViewer;
    use crate::utility::utility::get_line_ending_format;
    use crate::{
        enums::enums::{FindBarState, GoToState, ReplaceBarState},
        tab::tab::TextEditorTab,
    };

    pub struct StateManager {
        pub find_state: FindBarState,
        pub is_find_open: bool,
        pub find_val: String,
        pub replace_state: ReplaceBarState,
        pub is_replace_open: bool,
        pub is_replace_active: bool,
        pub replace_val: String,
        pub goto_state: GoToState,
        pub is_goto_open: bool,
        pub goto_val: String,
    }

    pub struct CursorIndexManager {
        pub start_idx: usize,
        pub end_idx: usize,
    }

    pub struct TextEditorApp {
        pub dock_state: DockState<TextEditorTab>,
        pub state_manager: StateManager,
        pub cursor_index_manager: CursorIndexManager,
        pub row_size: f32,
        pub pending_new_tab: bool,
        pub pending_close_tab: Option<usize>,
        pub pending_save_all: bool,
    }

    impl TextEditorApp {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            let status = format!("Ln 1 Col 1 | 100% | {} | UTF-8", get_line_ending_format());
            let dock_state = DockState::new(vec![TextEditorTab::new(
                "Untitled".into(),
                "".into(),
                status,
            )]);
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Self {
                dock_state,
                state_manager: StateManager {
                    find_state: FindBarState::NotFocused,
                    is_find_open: false,
                    find_val: String::new(),
                    replace_state: ReplaceBarState::NotFocused,
                    is_replace_open: false,
                    is_replace_active: false,
                    replace_val: String::new(),
                    goto_state: GoToState::NotFocused,
                    is_goto_open: false,
                    goto_val: String::new(),
                },
                cursor_index_manager: CursorIndexManager { start_idx: 0, end_idx: 0 },
                row_size: 16.0,
                pending_new_tab: false,
                pending_close_tab: None,
                pending_save_all: false,
            }
        }

        fn ui(&mut self, ui: &mut egui::Ui) {
            let mut style = Style::from_egui(ui.style());
            style.dock_area_padding = Some(egui::Margin { left: 0., right: 0., top: 0., bottom: 0. });
            style.main_surface_border_rounding = egui::Rounding { nw: 0., ne: 0., sw: 0., se: 0. };
            style.main_surface_border_stroke = egui::Stroke { width: 2.0, color: Color32::TRANSPARENT };
            style.tab_bar.bg_fill = Color32::TRANSPARENT;

            DockArea::new(&mut self.dock_state)
                .style(style)
                .show_add_buttons(true)
                .show_inside(ui, &mut MyTabViewer {
                    state_manager: &mut self.state_manager,
                    cursor_index_manager: &mut self.cursor_index_manager,
                    row_size: &mut self.row_size,
                    pending_new_tab: &mut self.pending_new_tab,
                    pending_close_tab: &mut self.pending_close_tab,
                    pending_save_all: &mut self.pending_save_all,
                });

            if self.pending_new_tab {
                self.pending_new_tab = false;
                self.dock_state
                    .main_surface_mut()
                    .root_node_mut()
                    .unwrap()
                    .append_tab(TextEditorTab::new(
                        "Untitled".into(),
                        "".into(),
                        format!("Ln 1 Col 1 | 100% | {} | UTF-8", get_line_ending_format()),
                    ));
            }
            if let Some(tab_id) = self.pending_close_tab.take() {
                let node = self.dock_state.main_surface_mut().root_node_mut().unwrap();
                if let Some(idx) = node.tabs().unwrap().iter().position(|t| t.id == tab_id) {
                    node.remove_tab(egui_dock::TabIndex(idx));
                }
            }
            if self.pending_save_all {
                self.pending_save_all = false;
                if let Some(tabs) = self.dock_state.main_surface_mut().root_node_mut().unwrap().tabs_mut() {
                    for tab in tabs {
                        tab.save();
                    }
                }
            }
        }
    }

    impl App for TextEditorApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                if ctx.input(|i| i.key_pressed(Key::F) && i.modifiers.ctrl) {
                    self.state_manager.is_find_open = true;
                    self.state_manager.find_state = FindBarState::Focused;
                }
                if ctx.input(|i| i.key_pressed(Key::R) && i.modifiers.ctrl) {
                    self.state_manager.is_find_open = true;
                    self.state_manager.find_state = FindBarState::NotFocused;
                    self.state_manager.is_replace_open = true;
                    self.state_manager.replace_state = ReplaceBarState::Focused;
                }
                let enter = ctx.input(|i| i.key_pressed(Key::Enter));

                let find_bar_state = self.state_manager.find_state.clone();
                let replace_bar_state = self.state_manager.replace_state.clone();

                match find_bar_state {
                    FindBarState::Finding => {
                        if !enter {
                            self.ui(ui);
                        } else {
                            self.cursor_index_manager.start_idx +=
                                self.state_manager.find_val.len();
                        }
                    }
                    FindBarState::NotFocused => match replace_bar_state {
                        ReplaceBarState::Replacing => {
                            if enter {
                                self.cursor_index_manager.start_idx +=
                                    self.state_manager.replace_val.len();
                                self.state_manager.is_replace_active = true;
                            } else {
                                self.ui(ui);
                            }
                        }
                        _ => self.ui(ui),
                    },
                    _ => self.ui(ui),
                }

                let status = self.dock_state
                    .main_surface()
                    .root_node()
                    .unwrap()
                    .tabs()
                    .unwrap()[0]
                    .status
                    .clone();

                TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                    ui.label(status);
                });
            });
        }
    }
}
