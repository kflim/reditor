pub mod editor {
    use std::cell::OnceCell;

    use eframe::App;
    use egui::CentralPanel;
    use egui::Color32;
    use egui::Key;
    use egui::Modifiers;
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
        find_state: FindBarState,
        is_find_open: bool,
        pub find_val: String,
        replace_state: ReplaceBarState,
        is_replace_open: bool,
        is_replace_active: bool,
        pub replace_val: String,
        goto_state: GoToState,
        is_goto_open: bool,
        pub goto_val: String,
    }

    impl StateManager {
        pub fn get_find_state(&self) -> FindBarState {
            self.find_state.clone()
        }

        pub fn set_find_state(&mut self, find_state: FindBarState) {
            match find_state {
                FindBarState::Finding => {
                    self.find_state = FindBarState::Finding;
                }
                FindBarState::Focused => {
                    self.find_state = FindBarState::Focused;
                }
                FindBarState::NotFocused => {
                    self.find_state = FindBarState::NotFocused;
                }
            }
        }

        pub fn get_is_find_open(&self) -> bool {
            self.is_find_open
        }

        pub fn set_is_find_open(&mut self, is_find_open: bool) {
            self.is_find_open = is_find_open;
        }

        pub fn get_find_val(&self) -> String {
            self.find_val.clone()
        }

        pub fn set_find_val(&mut self, find_val: String) {
            self.find_val = find_val;
        }

        pub fn get_replace_state(&self) -> ReplaceBarState {
            self.replace_state.clone()
        }

        pub fn set_replace_state(&mut self, replace_state: ReplaceBarState) {
            match replace_state {
                ReplaceBarState::Replacing => {
                    self.replace_state = ReplaceBarState::Replacing;
                }
                ReplaceBarState::Focused => {
                    self.replace_state = ReplaceBarState::Focused;
                }
                ReplaceBarState::NotFocused => {
                    self.replace_state = ReplaceBarState::NotFocused;
                }
            }
        }

        pub fn get_is_replace_open(&self) -> bool {
            self.is_replace_open
        }

        pub fn set_is_replace_open(&mut self, is_replace_open: bool) {
            self.is_replace_open = is_replace_open;
        }

        pub fn get_is_replace_active(&self) -> bool {
            self.is_replace_active
        }

        pub fn set_is_replace_active(&mut self, is_replace_active: bool) {
            self.is_replace_active = is_replace_active;
        }

        pub fn get_replace_val(&self) -> String {
            self.replace_val.clone()
        }

        pub fn set_replace_val(&mut self, replace_val: String) {
            self.replace_val = replace_val;
        }

        pub fn get_goto_state(&self) -> GoToState {
            self.goto_state.clone()
        }

        pub fn set_goto_state(&mut self, goto_state: GoToState) {
            match goto_state {
                GoToState::GoingTo => {
                    self.goto_state = GoToState::GoingTo;
                }
                GoToState::Focused => {
                    self.goto_state = GoToState::Focused;
                }
                GoToState::NotFocused => {
                    self.goto_state = GoToState::NotFocused;
                }
            }
        }

        pub fn get_is_goto_open(&self) -> bool {
            self.is_goto_open
        }

        pub fn set_is_goto_open(&mut self, is_goto_open: bool) {
            self.is_goto_open = is_goto_open;
        }

        pub fn get_goto_val(&self) -> String {
            self.goto_val.clone()
        }

        pub fn set_goto_val(&mut self, goto_val: String) {
            self.goto_val = goto_val;
        }
    }

    struct KeyManager {
        keys_pressed: Vec<Key>,
        modifiers_pressed: Vec<Modifiers>,
    }

    pub struct CursorIndexManager {
        start_idx: usize,
        end_idx: usize,
    }

    impl CursorIndexManager {
        pub fn get_start_idx(&self) -> usize {
            self.start_idx
        }

        pub fn set_start_idx(&mut self, start_idx: usize) {
            self.start_idx = start_idx;
        }

        pub fn get_end_idx(&self) -> usize {
            self.end_idx
        }

        pub fn set_end_idx(&mut self, end_idx: usize) {
            self.end_idx = end_idx;
        }
    }

    pub struct TextEditor {
        pub dock_state: DockState<TextEditorTab>,
        pub state_manager: StateManager,
        pub key_manager: KeyManager,
        pub cursor_index_manager: CursorIndexManager,
        pub row_size: f32,
    }

    impl TextEditor {
        pub fn set_find_open(&mut self) {
            if !self.state_manager.get_is_find_open() {
                self.state_manager.set_is_find_open(
                    self.key_manager.keys_pressed.contains(&Key::F)
                        && self
                            .key_manager
                            .modifiers_pressed
                            .contains(&Modifiers::CTRL),
                );
            }
            self.state_manager.set_find_state(FindBarState::Focused);
        }

        pub fn set_replace_open(&mut self) {
            if !self.state_manager.get_is_find_open() {
                self.state_manager.set_is_find_open(true);
            }
            self.state_manager.set_find_state(FindBarState::NotFocused);
            if !self.state_manager.get_is_replace_open() {
                self.state_manager.set_is_replace_open(
                    self.key_manager.keys_pressed.contains(&Key::R)
                        && self
                            .key_manager
                            .modifiers_pressed
                            .contains(&Modifiers::CTRL),
                );
            }
            self.state_manager
                .set_replace_state(ReplaceBarState::Focused);
        }
    }

    pub static mut TEXT_EDITOR: OnceCell<TextEditor> = OnceCell::new();

    pub struct TextEditorApp;

    impl TextEditorApp {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
            // Restore app state using cc.storage (requires the "persistence" feature).
            // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
            // for e.g. egui::PaintCallback.
            let status = format!("Ln 1 Col 1 | 100% | {} | UTF-8", get_line_ending_format());
            let editor = unsafe {
                TEXT_EDITOR.get_or_init(|| {
                    let dock_state = DockState::new(vec![TextEditorTab::new(
                        "Untitled".into(),
                        "".into(),
                        status,
                    )]);
                    TextEditor {
                        dock_state,
                        state_manager: StateManager {
                            find_state: FindBarState::NotFocused,
                            is_find_open: false,
                            find_val: "".to_string(),
                            replace_state: ReplaceBarState::NotFocused,
                            is_replace_open: false,
                            is_replace_active: false,
                            replace_val: "".to_string(),
                            goto_state: GoToState::NotFocused,
                            is_goto_open: false,
                            goto_val: "".to_string(),
                        },
                        key_manager: KeyManager {
                            keys_pressed: Vec::new(),
                            modifiers_pressed: Vec::new(),
                        },
                        cursor_index_manager: CursorIndexManager {
                            start_idx: 0,
                            end_idx: 0,
                        },
                        row_size: 16.0,
                    }
                })
            };
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            cc.egui_ctx.screen_rect().set_right(
                editor
                    .dock_state
                    .main_surface()
                    .root_node()
                    .unwrap()
                    .tabs()
                    .unwrap()[0]
                    .status
                    .len() as f32,
            );
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Self
        }

        fn ui(&mut self, ui: &mut egui::Ui) {
            let mut style = Style::from_egui(ui.style());

            style.dock_area_padding = Some(egui::Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            });
            style.main_surface_border_rounding = egui::Rounding {
                nw: 0.,
                ne: 0.,
                sw: 0.,
                se: 0.,
            };
            style.main_surface_border_stroke = egui::Stroke {
                width: 2.0,
                color: Color32::TRANSPARENT,
            };
            style.tab_bar.bg_fill = Color32::TRANSPARENT;

            DockArea::new(unsafe { &mut TEXT_EDITOR.get_mut().unwrap().dock_state })
                .style(style)
                .show_add_buttons(true)
                .show_inside(ui, &mut MyTabViewer);
        }
    }

    impl App for TextEditorApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                let mut is_enter_pressed = false;
                let events = ui.input(|i| i.raw.clone()).events.clone();
                for event in &events {
                    match event {
                        egui::Event::Key {
                            key,
                            physical_key: _,
                            pressed,
                            repeat: _,
                            modifiers,
                        } => {
                            if modifiers.ctrl {
                                if *pressed {
                                    unsafe {
                                        TEXT_EDITOR
                                            .get_mut()
                                            .unwrap()
                                            .key_manager
                                            .modifiers_pressed
                                            .push(Modifiers::CTRL)
                                    };
                                } else {
                                    unsafe {
                                        TEXT_EDITOR
                                            .get_mut()
                                            .unwrap()
                                            .key_manager
                                            .modifiers_pressed
                                            .retain(|&x| x != Modifiers::CTRL)
                                    };
                                }
                            }
                            match key {
                                Key::F => {
                                    if *pressed {
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .key_manager
                                                .keys_pressed
                                                .push(Key::F)
                                        };
                                        unsafe { TEXT_EDITOR.get_mut().unwrap().set_find_open() };
                                    } else {
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .key_manager
                                                .keys_pressed
                                                .retain(|&x| x != Key::F)
                                        };
                                    }
                                }
                                Key::R => {
                                    if *pressed {
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .key_manager
                                                .keys_pressed
                                                .push(Key::R)
                                        };
                                        unsafe {
                                            TEXT_EDITOR.get_mut().unwrap().set_replace_open()
                                        };
                                    } else {
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .key_manager
                                                .keys_pressed
                                                .retain(|&x| x != Key::R)
                                        };
                                    }
                                }
                                Key::Enter => {
                                    if *pressed {
                                        is_enter_pressed = true;
                                    } else {
                                        is_enter_pressed = false;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                let status = unsafe {
                    TEXT_EDITOR
                        .get()
                        .unwrap()
                        .dock_state
                        .main_surface()
                        .root_node()
                        .unwrap()
                        .tabs()
                        .unwrap()
                }[0]
                .status
                .clone();

                let find_bar_state = unsafe {
                    TEXT_EDITOR
                        .get_mut()
                        .unwrap()
                        .state_manager
                        .get_find_state()
                };

                let replace_bar_state = unsafe {
                    TEXT_EDITOR
                        .get_mut()
                        .unwrap()
                        .state_manager
                        .get_replace_state()
                };

                match find_bar_state {
                    FindBarState::Finding => {
                        if !is_enter_pressed {
                            self.ui(ui);
                        } else {
                            println!("Enter pressed find");
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .cursor_index_manager
                                    .start_idx += TEXT_EDITOR
                                    .get()
                                    .unwrap()
                                    .state_manager
                                    .get_find_val()
                                    .len()
                            };
                        }
                    }
                    FindBarState::NotFocused => match replace_bar_state {
                        ReplaceBarState::Replacing => {
                            if is_enter_pressed {
                                println!("Enter pressed replace");
                                unsafe {
                                    TEXT_EDITOR
                                        .get_mut()
                                        .unwrap()
                                        .cursor_index_manager
                                        .start_idx += TEXT_EDITOR
                                        .get()
                                        .unwrap()
                                        .state_manager
                                        .get_replace_val()
                                        .len()
                                };
                                unsafe {
                                    TEXT_EDITOR
                                        .get_mut()
                                        .unwrap()
                                        .state_manager
                                        .set_is_replace_active(true)
                                };
                            } else {
                                self.ui(ui);
                            }
                        }
                        _ => self.ui(ui),
                    },
                    _ => {
                        self.ui(ui);
                    }
                }

                TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                    ui.label(status);
                });
            });
        }
    }
}
