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
    use egui_dock::{TabIndex, TabViewer};

    use crate::editor::editor::TEXT_EDITOR;
    use crate::{
        enums::enums::{FindBarState, GoToState, ReplaceBarState},
        get_next_id,
        utility::utility::{get_line_ending_format, get_next_word_idx},
    };

    pub struct TextEditorTab {
        id: usize,
        file_path: String,
        title: String,
        text: String,
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
    }

    pub struct MyTabViewer;

    impl TabViewer for MyTabViewer {
        type Tab = TextEditorTab;

        fn on_add(&mut self, _surface: egui_dock::SurfaceIndex, _node: egui_dock::NodeIndex) {
            let surface = unsafe {
                TEXT_EDITOR
                    .get_mut()
                    .unwrap()
                    .dock_state
                    .get_surface_mut(_surface)
                    .unwrap()
            };
            let node = surface
                .node_tree_mut()
                .and_then(|node| node.root_node_mut());
            node.unwrap().append_tab(TextEditorTab::new(
                "Untitled".into(),
                "".into(),
                format!("Ln 1 Col 1 | 100% | {} | UTF-8", get_line_ending_format()),
            ));
        }

        fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
            if _tab.title == "+" {
                false
            } else {
                true
            }
        }

        fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
            tab.title.clone().into()
        }

        fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    // TODO : Add check for valid file path if file has previously existed
                    let save_file = |tab: &mut Self::Tab| {
                        if tab.file_path.clone().is_empty() {
                            let file = FileDialog::new()
                                .add_filter("Text documents", &["txt"])
                                .add_filter("Rust Source", &["rs"])
                                .add_filter("Python Source", &["py"])
                                .save_file();
                            if let Some(file) = file {
                                tab.file_path = file.as_path().to_str().unwrap().to_string();
                                tab.title = file.file_name().unwrap().to_str().unwrap().to_string();
                                tab.language =
                                    file.extension().unwrap().to_str().unwrap().to_string();
                                fs::write(file.clone(), tab.text.clone()).unwrap();
                            }
                        } else {
                            fs::write(tab.file_path.clone(), tab.text.clone()).unwrap();
                        }
                    };

                    if ui
                        .button("New Tab")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let node = unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .dock_state
                                .main_surface_mut()
                                .root_node_mut()
                                .unwrap()
                        };
                        node.append_tab(TextEditorTab::new(
                            "Untitled".into(),
                            "".into(),
                            format!("Ln 1 Col 1 | 100% | {} | UTF-8", get_line_ending_format()),
                        ));
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
                            tab.language = file.extension().unwrap().to_str().unwrap().to_string();
                            tab.is_refreshed = true;
                        }
                    }
                    if ui
                        .button("Save")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        save_file(tab);
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
                            tab.title = file.file_name().unwrap().to_str().unwrap().to_string();
                            tab.language = file.extension().unwrap().to_str().unwrap().to_string();
                            fs::write(file.clone(), tab.text.clone()).unwrap();
                        }
                    }
                    if ui
                        .button("Save all")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let tabs = unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .dock_state
                                .main_surface_mut()
                                .root_node_mut()
                                .unwrap()
                                .tabs_mut()
                                .unwrap()
                        };
                        for tab in tabs {
                            save_file(tab);
                        }
                    }
                    if ui
                        .button("Close tab")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let node = unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .dock_state
                                .main_surface_mut()
                                .root_node_mut()
                                .unwrap()
                        };
                        let tab_index = node
                            .tabs()
                            .unwrap()
                            .iter()
                            .position(|t| t.id == tab.id)
                            .unwrap();
                        node.remove_tab(TabIndex(tab_index));
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
                        let start_idx = unsafe {
                            TEXT_EDITOR
                                .get()
                                .unwrap()
                                .cursor_index_manager
                                .get_start_idx()
                        };
                        let end_idx = unsafe {
                            TEXT_EDITOR
                                .get()
                                .unwrap()
                                .cursor_index_manager
                                .get_end_idx()
                        };
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
                        let find_str = unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .state_manager
                                .get_find_val()
                                .clone()
                        };
                        let curr_start_idx = unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .cursor_index_manager
                                .get_start_idx()
                        };
                        let prev_word_idx = text[0..curr_start_idx].rfind(&find_str);
                        if let Some(idx) = prev_word_idx {
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .cursor_index_manager
                                    .set_start_idx(idx)
                            };
                        } else if let Some(idx) = text.rfind(&find_str) {
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .cursor_index_manager
                                    .set_start_idx(idx)
                            };
                        }
                        tab.is_refreshed = true;
                    }

                    if ui
                        .button("Go to")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .state_manager
                                .set_is_goto_open(true)
                        };
                        unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .state_manager
                                .set_goto_state(GoToState::Focused)
                        };
                    }

                    if ui
                        .button("Select all")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        ui.close_menu();
                        let text = tab.text.clone();
                        let len = text.len();
                        unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .cursor_index_manager
                                .set_start_idx(0)
                        };
                        unsafe {
                            TEXT_EDITOR
                                .get_mut()
                                .unwrap()
                                .cursor_index_manager
                                .set_end_idx(len)
                        };
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
                        println!("Fullscreen toggled");
                    }
                })
                .response
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            });

            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    &theme,
                    string,
                    &tab.language,
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| {
                    unsafe { TEXT_EDITOR.get_mut().unwrap().row_size = layout_job.font_height(f) };
                    f.layout_job(layout_job)
                })
            };

            let _find_window = Window::new("")
                .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 72.0)) // to change this magic value
                .fixed_size(Vec2::new(ui.available_width() / 2.0, 16.0))
                .title_bar(false)
                .open(
                    &mut unsafe { TEXT_EDITOR.get_mut().unwrap() }
                        .state_manager
                        .get_is_find_open(),
                )
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
                            egui::TextEdit::singleline(
                                &mut unsafe { TEXT_EDITOR.get_mut().unwrap() }
                                    .state_manager
                                    .find_val,
                            )
                            .hint_text("Find")
                            .desired_width(ui.available_width() - 32.0),
                        );
                        if let FindBarState::Focused =
                            unsafe { &TEXT_EDITOR.get().unwrap().state_manager.get_find_state() }
                        {
                            find_bar_response.request_focus();
                        }

                        if find_bar_response.gained_focus() {
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_find_state(FindBarState::Focused)
                            };
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_replace_state(ReplaceBarState::NotFocused)
                            };
                            tab.is_finding = true;
                        } else if find_bar_response.clicked_elsewhere() {
                            find_bar_response.surrender_focus();
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_find_state(FindBarState::NotFocused)
                            };
                            tab.is_finding = false;
                        }

                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            if tab.is_finding && !tab.is_replacing {
                                let find_state = unsafe {
                                    TEXT_EDITOR
                                        .get_mut()
                                        .unwrap()
                                        .state_manager
                                        .get_find_state()
                                };
                                match find_state {
                                    FindBarState::Focused => {
                                        find_bar_response.surrender_focus();
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .state_manager
                                                .set_find_state(FindBarState::Finding)
                                        };
                                    }
                                    _ => {}
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
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_is_find_open(false)
                            };
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_find_val("".into())
                            };
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_is_replace_open(false)
                            };
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_replace_val("".into())
                            };
                        }
                    });
                });

            let _replace_window = Window::new(" ")
                .anchor(Align2::CENTER_TOP, Vec2::new(35.0, 108.0)) // to change this magic value
                .fixed_size(Vec2::new(ui.available_width() / 2.0, 16.0))
                .title_bar(false)
                .open(
                    &mut unsafe { TEXT_EDITOR.get_mut().unwrap() }
                        .state_manager
                        .get_is_replace_open(),
                )
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
                            egui::TextEdit::singleline(
                                &mut unsafe { TEXT_EDITOR.get_mut().unwrap() }
                                    .state_manager
                                    .replace_val,
                            )
                            .hint_text("Replace")
                            .desired_width(ui.available_width()),
                        );
                        if let ReplaceBarState::Focused =
                            unsafe { &TEXT_EDITOR.get().unwrap().state_manager.get_replace_state() }
                        {
                            replace_bar_response.request_focus();
                        }
                        if replace_bar_response.gained_focus() {
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_find_state(FindBarState::NotFocused)
                            };
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_replace_state(ReplaceBarState::Focused)
                            };
                            tab.is_replacing = true;
                        } else if replace_bar_response.clicked_elsewhere() {
                            replace_bar_response.surrender_focus();
                            unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .set_replace_state(ReplaceBarState::NotFocused)
                            };
                            tab.is_replacing = false;
                        }

                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            let replace_state = unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .get_replace_state()
                            };

                            if tab.is_replacing {
                                match replace_state {
                                    ReplaceBarState::Focused => {
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .state_manager
                                                .set_replace_state(ReplaceBarState::Replacing)
                                        };
                                    }
                                    _ => {}
                                }
                            }
                        }

                        let replace_all_button = ui
                            .add(Button::new("Replace All"))
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if replace_all_button.clicked() {
                            let find_str = unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .get_find_val()
                                    .clone()
                            };
                            let replace_str = unsafe {
                                TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .get_replace_val()
                                    .clone()
                            };
                            let text = tab.text.clone();
                            let new_text = text.replace(&find_str, &replace_str);
                            tab.text = new_text;
                        }
                    });
                });

            /* let goto_window = Window::new("")
            .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 96.0)) // to change this magic value
            .fixed_size(Vec2::new(ui.available_width() / 2.0, 16.0))
            .title_bar(false)
            .open(&mut unsafe { TEXT_EDITOR.get_mut().unwrap() }.state_manager.get_is_goto_open())
            .show(ui.ctx(), |ui| {

            }); */

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

                let mut crange;
                if let Some(r) = text.cursor_range {
                    crange = Some(r);
                } else {
                    crange = None;
                }

                let find_bar_state = unsafe {
                    &TEXT_EDITOR
                        .get_mut()
                        .unwrap()
                        .state_manager
                        .get_find_state()
                };

                match find_bar_state {
                    FindBarState::Finding => {
                        text.response.request_focus();
                        if crange.is_some() {
                            let curr_start_idx = unsafe {
                                TEXT_EDITOR
                                    .get()
                                    .unwrap()
                                    .cursor_index_manager
                                    .get_start_idx()
                            };
                            let text_str = tab.text.clone();
                            let find_str = unsafe {
                                TEXT_EDITOR
                                    .get()
                                    .unwrap()
                                    .state_manager
                                    .get_find_val()
                                    .clone()
                            };
                            let find_result = get_next_word_idx(
                                &text_str,
                                unsafe {
                                    TEXT_EDITOR
                                        .get()
                                        .unwrap()
                                        .state_manager
                                        .get_find_val()
                                        .clone()
                                },
                                curr_start_idx,
                                ui.available_width() as usize,
                            );
                            if find_result.is_some() {
                                let next_word_idx = find_result.unwrap();
                                let mut new_range =
                                    CursorRange::one(text.cursor_range.clone().unwrap().primary);
                                new_range.primary.ccursor.index = next_word_idx.0 + find_str.len();
                                new_range.primary.rcursor.row = next_word_idx.1;
                                new_range.primary.rcursor.column = next_word_idx.2 + find_str.len();
                                new_range.primary.pcursor.paragraph = next_word_idx.3;
                                new_range.primary.pcursor.offset = next_word_idx.4 + find_str.len();
                                new_range.secondary.ccursor.index = next_word_idx.0;
                                new_range.secondary.rcursor.row = next_word_idx.1;
                                new_range.secondary.rcursor.column = next_word_idx.2;
                                new_range.secondary.pcursor.paragraph = next_word_idx.3;
                                new_range.secondary.pcursor.offset = next_word_idx.4;
                                let mut crange = text.cursor_range.unwrap();
                                crange.primary = new_range.primary;
                                crange.secondary = new_range.secondary;
                                text.cursor_range = Some(crange);
                                text.state.cursor.set_range(Some(crange));
                                text.state.store(ui.ctx(), text.response.id);
                                unsafe {
                                    TEXT_EDITOR
                                        .get_mut()
                                        .unwrap()
                                        .cursor_index_manager
                                        .set_start_idx(next_word_idx.0)
                                };
                                unsafe {
                                    TEXT_EDITOR
                                        .get_mut()
                                        .unwrap()
                                        .cursor_index_manager
                                        .set_end_idx(next_word_idx.0 + find_str.len())
                                };
                                let row_height = unsafe { TEXT_EDITOR.get().unwrap().row_size };

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
                                tab.error_msg = format!("Cannot find \"{}\"", find_str).into();
                            }
                        }
                    }
                    FindBarState::NotFocused => {
                        if !unsafe {
                            TEXT_EDITOR
                                .get()
                                .unwrap()
                                .state_manager
                                .get_is_replace_open()
                        } {
                            text.response.request_focus();
                            if crange.is_some() {
                                let primary_idx = crange.unwrap().primary.ccursor.index;
                                let secondary_idx = crange.unwrap().secondary.ccursor.index;
                                if primary_idx != secondary_idx {
                                    unsafe {
                                        TEXT_EDITOR
                                            .get_mut()
                                            .unwrap()
                                            .cursor_index_manager
                                            .set_start_idx(min(primary_idx, secondary_idx))
                                    };
                                    unsafe {
                                        TEXT_EDITOR
                                            .get_mut()
                                            .unwrap()
                                            .cursor_index_manager
                                            .set_end_idx(max(primary_idx, secondary_idx))
                                    };
                                }
                            }
                        } else {
                            let replace_bar_state = unsafe {
                                &TEXT_EDITOR
                                    .get_mut()
                                    .unwrap()
                                    .state_manager
                                    .get_replace_state()
                            };

                            match replace_bar_state {
                                ReplaceBarState::Replacing => {
                                    text.response.request_focus();
                                    if let Some(r) = text.cursor_range {
                                        crange = Some(r);
                                    } else {
                                        crange = None;
                                    }
                                    if crange.is_some() {
                                        let curr_start_idx = unsafe {
                                            TEXT_EDITOR
                                                .get()
                                                .unwrap()
                                                .cursor_index_manager
                                                .get_start_idx()
                                        };
                                        if curr_start_idx == 0 {
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get_mut()
                                                    .unwrap()
                                                    .state_manager
                                                    .set_is_replace_active(true)
                                            };
                                        }
                                        if !unsafe {
                                            TEXT_EDITOR
                                                .get()
                                                .unwrap()
                                                .state_manager
                                                .get_is_replace_active()
                                        } {
                                            return;
                                        }
                                        let find_str = unsafe {
                                            TEXT_EDITOR
                                                .get()
                                                .unwrap()
                                                .state_manager
                                                .get_find_val()
                                                .clone()
                                        };
                                        let replace_str = unsafe {
                                            TEXT_EDITOR
                                                .get()
                                                .unwrap()
                                                .state_manager
                                                .get_replace_val()
                                                .clone()
                                        };
                                        if curr_start_idx > 0 {
                                            let before = tab.text
                                                [0..curr_start_idx - 1 - find_str.len()]
                                                .to_string();
                                            let after = tab.text[curr_start_idx - 1..].to_string();
                                            tab.text = before + &replace_str + &after;
                                        }
                                        let text_str = tab.text.clone();
                                        let find_result = get_next_word_idx(
                                            &text_str,
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get()
                                                    .unwrap()
                                                    .state_manager
                                                    .get_find_val()
                                                    .clone()
                                            },
                                            curr_start_idx,
                                            ui.available_width() as usize,
                                        );
                                        if find_result.is_some() {
                                            let next_word_idx = find_result.unwrap();
                                            let mut new_range = CursorRange::one(
                                                text.cursor_range.clone().unwrap().primary,
                                            );
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
                                            let mut crange = text.cursor_range.unwrap();
                                            crange.primary = new_range.primary;
                                            crange.secondary = new_range.secondary;
                                            text.cursor_range = Some(crange);
                                            text.state.cursor.set_range(Some(crange));
                                            text.state.store(ui.ctx(), text.response.id);
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get_mut()
                                                    .unwrap()
                                                    .cursor_index_manager
                                                    .set_start_idx(next_word_idx.0)
                                            };
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get_mut()
                                                    .unwrap()
                                                    .cursor_index_manager
                                                    .set_end_idx(next_word_idx.0 + find_str.len())
                                            };
                                            let row_height =
                                                unsafe { TEXT_EDITOR.get().unwrap().row_size };

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
                                                format!("Cannot find \"{}\"", find_str).into();
                                        }
                                        unsafe {
                                            TEXT_EDITOR
                                                .get_mut()
                                                .unwrap()
                                                .state_manager
                                                .set_is_replace_active(false)
                                        };
                                    }
                                }
                                ReplaceBarState::NotFocused => {
                                    text.response.request_focus();
                                    if crange.is_some() {
                                        let primary_idx = crange.unwrap().primary.ccursor.index;
                                        let secondary_idx = crange.unwrap().secondary.ccursor.index;
                                        if primary_idx != secondary_idx {
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get_mut()
                                                    .unwrap()
                                                    .cursor_index_manager
                                                    .set_start_idx(min(primary_idx, secondary_idx))
                                            };
                                            unsafe {
                                                TEXT_EDITOR
                                                    .get_mut()
                                                    .unwrap()
                                                    .cursor_index_manager
                                                    .set_end_idx(max(primary_idx, secondary_idx))
                                            };
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

            if tab.has_error {
                let text_color;
                if theme == CodeTheme::dark() {
                    text_color = Color32::WHITE;
                } else {
                    text_color = Color32::BLACK;
                }
                let _error_window = Window::new(RichText::new("Reditor").color(text_color))
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .min_width(250.0)
                    .min_height(150.0)
                    .show(ui.ctx(), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label(RichText::new(tab.error_msg.clone()).color(text_color).font(
                                FontId {
                                    size: 14.0,
                                    family: FontFamily::Proportional,
                                },
                            ));
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
