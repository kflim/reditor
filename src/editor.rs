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
  use crate::{enums::enums::{FindBarState, GoToState, ReplaceBarState}, tab::tab::TextEditorTab, KeyManager};

  pub struct TextEditor {
    pub dock_state: DockState<TextEditorTab>,
    pub find_str: String,
    pub replace_str: String,
    pub is_find_open: bool,
    pub is_replace_open: bool,
    pub is_goto_open: bool,
    pub find_state: FindBarState,
    pub replace_state: ReplaceBarState,
    pub goto_state: GoToState,
    pub key_manager: KeyManager,
    pub curr_start_idx: usize,
    pub curr_end_idx: usize,
    pub row_size: f32,
  }

  impl TextEditor {
      pub fn set_find_open(&mut self) {
          if !self.is_find_open {
              self.is_find_open = self.key_manager.keys_pressed.contains(&Key::F) && self.key_manager.modifiers_pressed.contains(&Modifiers::CTRL)
          }
          self.find_state = FindBarState::Focused;
      }

      pub fn set_replace_open(&mut self) {
          self.is_find_open = true;
          if !self.is_replace_open {
              self.is_replace_open = self.key_manager.keys_pressed.contains(&Key::R) && self.key_manager.modifiers_pressed.contains(&Modifiers::CTRL)
          }
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
          let editor = unsafe { TEXT_EDITOR.get_or_init(|| {
              let dock_state = DockState::new(vec![
                  TextEditorTab::new("Untitled".into(), "".into(), status),
              ]);
              TextEditor { 
                  dock_state,
                  find_str: "".into(),
                  replace_str: "".into(),
                  is_find_open: false,
                  is_replace_open: false,
                  is_goto_open: false,
                  find_state: FindBarState::Closed,
                  replace_state: ReplaceBarState::Closed,
                  goto_state: GoToState::Closed,
                  key_manager: KeyManager { keys_pressed: Vec::new(), modifiers_pressed: Vec::new()},
                  curr_start_idx: 0,
                  curr_end_idx: 0,
                  row_size: 16.0,
              }
          }) };
          cc.egui_ctx.set_visuals(egui::Visuals::dark());
          cc.egui_ctx.screen_rect().set_right(
              editor.dock_state
                  .main_surface()
                  .root_node()
                  .unwrap()
                  .tabs()
                  .unwrap()[0]
                  .status.len() as f32
          );
          egui_extras::install_image_loaders(&cc.egui_ctx);

          Self
      }

      fn ui(&mut self, ui: &mut egui::Ui) {
          let mut style = Style::from_egui(ui.style());

          style.dock_area_padding = Some(egui::Margin { left: 0., right: 0., top: 0., bottom: 0.});
          style.main_surface_border_rounding = egui::Rounding { nw: 0., ne: 0., sw: 0., se: 0. };
          style.main_surface_border_stroke = egui::Stroke {width: 2.0, color: Color32::TRANSPARENT};
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
                      egui::Event::Key { key, physical_key: _, pressed, repeat: _, modifiers } => {
                          if modifiers.ctrl {
                              if *pressed {
                                  unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.modifiers_pressed.push(Modifiers::CTRL) };
                              } else {
                                  unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.modifiers_pressed.retain(|&x| x != Modifiers::CTRL) };
                              }
                          }
                          match key {
                              Key::F => {
                                  if *pressed {
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.keys_pressed.push(Key::F) };
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().set_find_open() };
                                  } else {
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.keys_pressed.retain(|&x| x != Key::F) };
                                  }
                              },
                              Key::R => {
                                  if *pressed {
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.keys_pressed.push(Key::R) };
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().set_replace_open() };
                                      println!("R pressed")
                                  } else {
                                      unsafe { TEXT_EDITOR.get_mut().unwrap().key_manager.keys_pressed.retain(|&x| x != Key::R) };
                                  }
                              },
                              Key::Enter => {
                                  if *pressed {
                                      is_enter_pressed = true;
                                      println!("Enter pressed");
                                  }
                              }
                              _ => {}
                          }
                      },
                      _ => {}
                  }
              }

              let status = 
                  unsafe { 
                      TEXT_EDITOR.get()
                          .unwrap()
                          .dock_state
                          .main_surface()
                          .root_node()
                          .unwrap()
                          .tabs()
                          .unwrap() 
                  }[0].status.clone();

              let find_bar_state = unsafe { &TEXT_EDITOR.get_mut().unwrap().find_state };
              match find_bar_state {
                  FindBarState::Finding => {
                      if !is_enter_pressed {
                          self.ui(ui);
                      } else {
                          unsafe { TEXT_EDITOR.get_mut().unwrap().curr_start_idx += TEXT_EDITOR.get().unwrap().find_str.len() };
                      }
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