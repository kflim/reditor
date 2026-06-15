pub mod tab {
    use chardet::detect;
    use egui_extras::syntax_highlighting::CodeTheme;
    use rfd::FileDialog;
    use std::{
        cmp::{max, min},
        fs,
        ops::Range,
    };

    use egui::{
        text::CursorRange,
        text_selection::{text_cursor_state::cursor_rect, visuals::paint_text_selection},
        Align2, Button, Color32, FontFamily, FontId, ImageButton, Key, RichText, ScrollArea,
        TextBuffer, Vec2, Widget, Window,
    };
    use egui_dock::TabViewer;

    use crate::editor::editor::{CursorIndexManager, StateManager};
    use crate::{
        enums::enums::{FindBarState, GoToState, ReplaceBarState},
        utility::utility::{get_line_ending_format, get_next_id, get_next_word_idx},
    };

    pub struct TextEditorTab {
        pub id: usize,
        file_path: String,
        title: String,
        pub text: String,
        pub status: String,
        language: String,
        dirty: bool,
        is_refreshed: bool,
        is_finding: bool,
        is_replacing: bool,
        has_error: bool,
        error_msg: String,
    }

    impl TextEditorTab {
        pub fn new(title: String, text: String, status: String) -> Self {
            Self {
                id: get_next_id(),
                file_path: "".into(),
                title,
                text,
                status,
                language: "".into(),
                dirty: false,
                is_refreshed: false,
                is_finding: false,
                is_replacing: false,
                has_error: false,
                error_msg: "".into(),
            }
        }

        pub fn save(&mut self) {
            if self.file_path.is_empty() {
                let file = FileDialog::new()
                    .add_filter("Text documents", &["txt"])
                    .add_filter("Rust Source", &["rs"])
                    .add_filter("Python Source", &["py"])
                    .save_file();
                if let Some(file) = file {
                    self.file_path = file.as_path().to_str().unwrap().to_string();
                    self.title = file.file_name().unwrap().to_str().unwrap().to_string();
                    self.language = file.extension().unwrap().to_str().unwrap().to_string();
                    fs::write(file, self.text.clone()).unwrap();
                }
            } else {
                fs::write(self.file_path.clone(), self.text.clone()).unwrap();
            }
        }
    }

    pub struct MyTabViewer<'a> {
        pub state_manager: &'a mut StateManager,
        pub cursor_index_manager: &'a mut CursorIndexManager,
        pub row_size: &'a mut f32,
        pub pending_new_tab: &'a mut bool,
        pub pending_close_tab: &'a mut Option<usize>,
        pub pending_save_all: &'a mut bool,
    }

    impl TabViewer for MyTabViewer<'_> {
        type Tab = TextEditorTab;

        fn on_add(&mut self, _surface: egui_dock::SurfaceIndex, _node: egui_dock::NodeIndex) {
            *self.pending_new_tab = true;
        }

        fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
            _tab.title != "+"
        }

        fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
            tab.title.clone().into()
        }

        fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button("New Tab")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        *self.pending_new_tab = true;
                    }
                    if ui
                        .button("Open")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let file = FileDialog::new().pick_file();
                        if let Some(file) = file {
                            let text = std::fs::read_to_string(file.clone()).unwrap();
                            tab.text = text.clone();
                            tab.title = file.file_name().unwrap().to_str().unwrap().to_string();
                            let encoding = detect(text.as_bytes()).0;
                            tab.status = format!(
                                "Ln 1 Col 1 | 100% | {} | {}",
                                get_line_ending_format(),
                                encoding
                            );
                            tab.language =
                                file.extension().unwrap().to_str().unwrap().to_string();
                            tab.is_refreshed = true;
                        }
                    }
                    if ui
                        .button("Save")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        tab.save();
                    }
                    if ui
                        .button("Save As")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let file = FileDialog::new()
                            .add_filter("Text documents", &["txt"])
                            .add_filter("Rust Source", &["rs"])
                            .add_filter("Python Source", &["py"])
                            .save_file();
                        if let Some(file) = file {
                            tab.file_path = file.as_path().to_str().unwrap().to_string();
                            tab.title =
                                file.file_name().unwrap().to_str().unwrap().to_string();
                            tab.language =
                                file.extension().unwrap().to_str().unwrap().to_string();
                            fs::write(file.clone(), tab.text.clone()).unwrap();
                        }
                    }
                    if ui
                        .button("Save all")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        *self.pending_save_all = true;
                    }
                    if ui
                        .button("Close tab")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        *self.pending_close_tab = Some(tab.id);
                    }
                    if ui
                        .button("Close window")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        std::process::exit(0);
                    }
                })
                .response
                .on_hover_cursor(egui::CursorIcon::PointingHand);

                ui.menu_button("Edit", |ui| {
                    if ui
                        .button("Cut")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let start_idx = self.cursor_index_manager.start_idx;
                        let end_idx = self.cursor_index_manager.end_idx;
                        ui.ctx().copy_text(tab.text[start_idx..end_idx].to_string());
                        tab.text.delete_char_range(Range {
                            start: start_idx,
                            end: end_idx,
                        });
                    }

                    if ui
                        .button("Find previous")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let text = tab.text.clone();
                        let find_str = self.state_manager.find_val.clone();
                        let curr_start_idx = self.cursor_index_manager.start_idx;
                        let prev_word_idx = text[0..curr_start_idx].rfind(&find_str);
                        if let Some(idx) = prev_word_idx {
                            self.cursor_index_manager.start_idx = idx;
                        } else if let Some(idx) = text.rfind(&find_str) {
                            self.cursor_index_manager.start_idx = idx;
                        }
                        tab.is_refreshed = true;
                    }

                    if ui
                        .button("Go to")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        self.state_manager.is_goto_open = true;
                        self.state_manager.goto_state = GoToState::Focused;
                    }

                    if ui
                        .button("Select all")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let len = tab.text.len();
                        self.cursor_index_manager.start_idx = 0;
                        self.cursor_index_manager.end_idx = len;
                        tab.is_refreshed = true;
                    }
                })
                .response
                .on_hover_cursor(egui::CursorIcon::PointingHand);

                ui.menu_button("View", |ui| {
                    if ui
                        .button("Toggle Fullscreen")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                    }
                })
                .response
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            });

            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

            let row_height = *self.row_size;
            let mut updated_row_size = row_height;

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    &theme,
                    string,
                    &tab.language,
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| {
                    updated_row_size = layout_job.font_height(f);
                    f.layout_job(layout_job)
                })
            };

            let mut is_find_open = self.state_manager.is_find_open;
            let _find_window = Window::new("")
                .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 72.0))
                .fixed_size(Vec2::new(ui.available_width() / 2.0, 16.0))
                .title_bar(false)
                .open(&mut is_find_open)
                .show(ui.ctx(), |ui| {
                    let ui_visuals = ui.visuals_mut();
                    ui_visuals.selection.stroke = egui::Stroke {
                        width: 0.0,
                        color: Color32::TRANSPARENT,
                    };
                    ui_visuals.widgets.hovered.bg_stroke = egui::Stroke {
                        width: 0.0,
                        color: Color32::TRANSPARENT,
                    };

                    ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 8.0);

                    ui.horizontal(|ui| {
                        let find_bar_response = ui.add(
                            egui::TextEdit::singleline(&mut self.state_manager.find_val)
                                .hint_text("Find")
                                .desired_width(ui.available_width() - 32.0),
                        );
                        if let FindBarState::Focused = self.state_manager.find_state {
                            find_bar_response.request_focus();
                        }

                        if find_bar_response.gained_focus() {
                            self.state_manager.find_state = FindBarState::Focused;
                            self.state_manager.replace_state = ReplaceBarState::NotFocused;
                            tab.is_finding = true;
                        } else if find_bar_response.clicked_elsewhere() {
                            find_bar_response.surrender_focus();
                            self.state_manager.find_state = FindBarState::NotFocused;
                            tab.is_finding = false;
                        }

                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            if tab.is_finding && !tab.is_replacing {
                                if let FindBarState::Focused = self.state_manager.find_state {
                                    find_bar_response.surrender_focus();
                                    self.state_manager.find_state = FindBarState::Finding;
                                }
                            }
                        }

                        let find_button =
                            ImageButton::new(egui::include_image!("../assets/find.png"));
                        ui.style_mut().spacing.button_padding = Vec2::new(-1.0, -1.0);
                        ui.style_mut().visuals.selection.stroke = egui::Stroke {
                            width: 0.0,
                            color: Color32::TRANSPARENT,
                        };
                        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: Color32::TRANSPARENT,
                        };

                        if find_button
                            .ui(ui)
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            // To finish
                        }

                        let close_button =
                            ImageButton::new(egui::include_image!("../assets/close.png"));
                        ui.style_mut().spacing.button_padding = Vec2::new(-1.0, -1.0);
                        ui.style_mut().visuals.selection.stroke = egui::Stroke {
                            width: 0.0,
                            color: Color32::TRANSPARENT,
                        };
                        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke {
                            width: 0.0,
                            color: Color32::TRANSPARENT,
                        };

                        if close_button
                            .ui(ui)
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.state_manager.is_find_open = false;
                            self.state_manager.find_val = "".into();
                            self.state_manager.is_replace_open = false;
                            self.state_manager.replace_val = "".into();
                        }
                    });
                });
            self.state_manager.is_find_open = is_find_open;

            let mut is_replace_open = self.state_manager.is_replace_open;
            let _replace_window = Window::new(" ")
                .anchor(Align2::CENTER_TOP, Vec2::new(35.0, 108.0))
                .fixed_size(Vec2::new(ui.available_width() / 2.0, 16.0))
                .title_bar(false)
                .open(&mut is_replace_open)
                .show(ui.ctx(), |ui| {
                    let ui_visuals = ui.visuals_mut();
                    ui_visuals.selection.stroke = egui::Stroke {
                        width: 0.0,
                        color: Color32::TRANSPARENT,
                    };
                    ui_visuals.widgets.hovered.bg_stroke = egui::Stroke {
                        width: 0.0,
                        color: Color32::TRANSPARENT,
                    };

                    ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 8.0);

                    ui.horizontal(|ui| {
                        let replace_bar_response = ui.add(
                            egui::TextEdit::singleline(&mut self.state_manager.replace_val)
                                .hint_text("Replace")
                                .desired_width(ui.available_width()),
                        );
                        if let ReplaceBarState::Focused = self.state_manager.replace_state {
                            replace_bar_response.request_focus();
                        }
                        if replace_bar_response.gained_focus() {
                            self.state_manager.find_state = FindBarState::NotFocused;
                            self.state_manager.replace_state = ReplaceBarState::Focused;
                            tab.is_replacing = true;
                        } else if replace_bar_response.clicked_elsewhere() {
                            replace_bar_response.surrender_focus();
                            self.state_manager.replace_state = ReplaceBarState::NotFocused;
                            tab.is_replacing = false;
                        }

                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            if tab.is_replacing {
                                if let ReplaceBarState::Focused = self.state_manager.replace_state
                                {
                                    self.state_manager.replace_state = ReplaceBarState::Replacing;
                                }
                            }
                        }

                        let replace_all_button = ui
                            .add(Button::new("Replace All"))
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if replace_all_button.clicked() {
                            let find_str = self.state_manager.find_val.clone();
                            let replace_str = self.state_manager.replace_val.clone();
                            tab.text = tab.text.replace(&find_str, &replace_str);
                        }
                    });
                });
            self.state_manager.is_replace_open = is_replace_open;

            ScrollArea::both().show(ui, |ui| {
                let ui_visuals = ui.visuals_mut();
                ui_visuals.selection.stroke = egui::Stroke {
                    width: 2.0,
                    color: Color32::TRANSPARENT,
                };
                ui_visuals.widgets.hovered.bg_stroke = egui::Stroke {
                    width: 2.0,
                    color: Color32::TRANSPARENT,
                };

                let mut text = egui::TextEdit::multiline(&mut tab.text)
                    .code_editor()
                    .layouter(&mut layouter)
                    .min_size(ui.available_size())
                    .desired_width(ui.available_width())
                    .cursor_at_end(false)
                    .show(ui);

                if tab.is_refreshed {
                    tab.is_refreshed = false;
                    text.response.request_focus();
                }

                let mut crange = text.cursor_range;

                match self.state_manager.find_state.clone() {
                    FindBarState::Finding => {
                        text.response.request_focus();
                        if crange.is_some() {
                            let curr_start_idx = self.cursor_index_manager.start_idx;
                            let text_str = tab.text.clone();
                            let find_str = self.state_manager.find_val.clone();
                            let find_result = get_next_word_idx(
                                &text_str,
                                find_str.clone(),
                                curr_start_idx,
                                ui.available_width() as usize,
                            );
                            if let Some(next_word_idx) = find_result {
                                let mut new_range =
                                    CursorRange::one(text.cursor_range.clone().unwrap().primary);
                                new_range.primary.ccursor.index =
                                    next_word_idx.0 + find_str.len();
                                new_range.primary.rcursor.row = next_word_idx.1;
                                new_range.primary.rcursor.column =
                                    next_word_idx.2 + find_str.len();
                                new_range.primary.pcursor.paragraph = next_word_idx.3;
                                new_range.primary.pcursor.offset =
                                    next_word_idx.4 + find_str.len();
                                new_range.secondary.ccursor.index = next_word_idx.0;
                                new_range.secondary.rcursor.row = next_word_idx.1;
                                new_range.secondary.rcursor.column = next_word_idx.2;
                                new_range.secondary.pcursor.paragraph = next_word_idx.3;
                                new_range.secondary.pcursor.offset = next_word_idx.4;
                                let mut cr = text.cursor_range.unwrap();
                                cr.primary = new_range.primary;
                                cr.secondary = new_range.secondary;
                                text.cursor_range = Some(cr);
                                text.state.cursor.set_range(Some(cr));
                                text.state.store(ui.ctx(), text.response.id);
                                self.cursor_index_manager.start_idx = next_word_idx.0;
                                self.cursor_index_manager.end_idx =
                                    next_word_idx.0 + find_str.len();
                                let crect = cursor_rect(
                                    text.galley_pos,
                                    &text.galley,
                                    &text.cursor_range.unwrap().primary,
                                    row_height,
                                );
                                ui.scroll_to_rect(crect, None);
                                paint_text_selection(
                                    ui.painter(),
                                    ui.visuals(),
                                    text.galley_pos,
                                    &text.galley,
                                    &text.cursor_range.unwrap(),
                                    None,
                                );
                            } else {
                                tab.has_error = true;
                                tab.error_msg = format!("Cannot find \"{}\"", find_str);
                            }
                        }
                    }
                    FindBarState::NotFocused => {
                        if !self.state_manager.is_replace_open {
                            text.response.request_focus();
                            if let Some(cr) = crange {
                                let primary_idx = cr.primary.ccursor.index;
                                let secondary_idx = cr.secondary.ccursor.index;
                                if primary_idx != secondary_idx {
                                    self.cursor_index_manager.start_idx =
                                        min(primary_idx, secondary_idx);
                                    self.cursor_index_manager.end_idx =
                                        max(primary_idx, secondary_idx);
                                }
                            }
                        } else {
                            match self.state_manager.replace_state.clone() {
                                ReplaceBarState::Replacing => {
                                    text.response.request_focus();
                                    crange = text.cursor_range;
                                    if crange.is_some() {
                                        let curr_start_idx =
                                            self.cursor_index_manager.start_idx;
                                        if curr_start_idx == 0 {
                                            self.state_manager.is_replace_active = true;
                                        }
                                        if !self.state_manager.is_replace_active {
                                            return;
                                        }
                                        let find_str = self.state_manager.find_val.clone();
                                        let replace_str =
                                            self.state_manager.replace_val.clone();
                                        if curr_start_idx > 0 {
                                            let before = tab.text
                                                [0..curr_start_idx - 1 - find_str.len()]
                                                .to_string();
                                            let after =
                                                tab.text[curr_start_idx - 1..].to_string();
                                            tab.text = before + &replace_str + &after;
                                        }
                                        let text_str = tab.text.clone();
                                        let find_result = get_next_word_idx(
                                            &text_str,
                                            find_str.clone(),
                                            curr_start_idx,
                                            ui.available_width() as usize,
                                        );
                                        if let Some(next_word_idx) = find_result {
                                            let mut new_range = CursorRange::one(
                                                text.cursor_range.clone().unwrap().primary,
                                            );
                                            new_range.primary.ccursor.index =
                                                next_word_idx.0 + find_str.len();
                                            new_range.primary.rcursor.row = next_word_idx.1;
                                            new_range.primary.rcursor.column =
                                                next_word_idx.2 + find_str.len();
                                            new_range.primary.pcursor.paragraph =
                                                next_word_idx.3;
                                            new_range.primary.pcursor.offset =
                                                next_word_idx.4 + find_str.len();
                                            new_range.secondary.ccursor.index = next_word_idx.0;
                                            new_range.secondary.rcursor.row = next_word_idx.1;
                                            new_range.secondary.rcursor.column = next_word_idx.2;
                                            new_range.secondary.pcursor.paragraph =
                                                next_word_idx.3;
                                            new_range.secondary.pcursor.offset = next_word_idx.4;
                                            let mut cr = text.cursor_range.unwrap();
                                            cr.primary = new_range.primary;
                                            cr.secondary = new_range.secondary;
                                            text.cursor_range = Some(cr);
                                            text.state.cursor.set_range(Some(cr));
                                            text.state.store(ui.ctx(), text.response.id);
                                            self.cursor_index_manager.start_idx = next_word_idx.0;
                                            self.cursor_index_manager.end_idx =
                                                next_word_idx.0 + find_str.len();
                                            let crect = cursor_rect(
                                                text.galley_pos,
                                                &text.galley,
                                                &text.cursor_range.unwrap().primary,
                                                row_height,
                                            );
                                            ui.scroll_to_rect(crect, None);
                                            paint_text_selection(
                                                ui.painter(),
                                                ui.visuals(),
                                                text.galley_pos,
                                                &text.galley,
                                                &text.cursor_range.unwrap(),
                                                None,
                                            );
                                        } else {
                                            tab.has_error = true;
                                            tab.error_msg =
                                                format!("Cannot find \"{}\"", find_str);
                                        }
                                        self.state_manager.is_replace_active = false;
                                    }
                                }
                                ReplaceBarState::NotFocused => {
                                    text.response.request_focus();
                                    if let Some(cr) = crange {
                                        let primary_idx = cr.primary.ccursor.index;
                                        let secondary_idx = cr.secondary.ccursor.index;
                                        if primary_idx != secondary_idx {
                                            self.cursor_index_manager.start_idx =
                                                min(primary_idx, secondary_idx);
                                            self.cursor_index_manager.end_idx =
                                                max(primary_idx, secondary_idx);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }

                if text.response.changed() {
                    tab.dirty = true;
                }
            });

            *self.row_size = updated_row_size;

            if tab.has_error {
                let text_color = if theme == CodeTheme::dark() {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                };
                let _error_window = Window::new(RichText::new("Reditor").color(text_color))
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .min_width(250.0)
                    .min_height(150.0)
                    .show(ui.ctx(), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(tab.error_msg.clone()).color(text_color).font(
                                    FontId {
                                        size: 14.0,
                                        family: FontFamily::Proportional,
                                    },
                                ),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.style_mut().spacing.button_padding = Vec2::new(8.0, 8.0);
                            if ui
                                .button(RichText::new("OK").color(text_color))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                tab.has_error = false;
                            }
                        });
                    })
                    .unwrap()
                    .response
                    .request_focus();
            }
        }
    }
}
