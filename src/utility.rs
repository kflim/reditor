pub mod utility {
  use std::cmp;
  use std::env;
  use std::sync::atomic::{AtomicUsize, Ordering};

  use crate::editor::editor::TEXT_EDITOR;

  pub fn get_line_ending_format() -> String {
    let os = env::consts::OS;

    let line_ending = match os {
        "windows" => "CRLF",
        "macos" => "CR",
        "linux" => "LF",
        _ => "LF"
    };

    let formal_os = match os {
        "windows" => "Windows",
        "macos" => "macOS",
        "linux" => "Linux",
        _ => "Unknown"
    };

    format!("{} ({})", if formal_os == "Unknown" { os } else { formal_os }, line_ending)
  }

  pub fn get_next_word_idx(text: &str, curr_start_idx: usize, ui_width: usize) -> (usize, usize, usize, usize, usize) {
    let mut ccursor_index = 0;
    let pcursor_offset ;
    let pcursor_paragraph;
    let rcursor_column;
    let mut rcursor_row = 0;
    let find_str = unsafe { TEXT_EDITOR.get().unwrap().find_str.clone() };

    // Find pure offset
    let mut word_idx = text[curr_start_idx..].find(&find_str);
    if word_idx.is_none() {
        word_idx = text[0..curr_start_idx].find(&find_str);
    } else {
        word_idx = Some(word_idx.unwrap() + curr_start_idx);
    }

    if let Some(idx) = word_idx {
        ccursor_index = idx;
    }
    
    // Find paragraph idx and offset within paragraph
    pcursor_paragraph = text.chars().take(ccursor_index).filter(|&c| c == '\n').count();
    let paragraphs = text.split("\n").collect::<Vec<&str>>();
    let cur_paragraph = paragraphs[pcursor_paragraph].to_string();
    if let Some(offset) = cur_paragraph.find(&find_str) {
        pcursor_offset = offset;
    } else {
        pcursor_offset = 0;
    }

    // Find row idx and col idx
    rcursor_column = pcursor_offset % ui_width;
    for i in 0..pcursor_paragraph {
        rcursor_row += cmp::max(1, paragraphs[i].len() / ui_width);
    }
    rcursor_row += cmp::max(1, pcursor_offset / ui_width) - 1;

    (ccursor_index, rcursor_row, rcursor_column, pcursor_paragraph, pcursor_offset)
  }

  static mut COUNTER: AtomicUsize = AtomicUsize::new(0);

  pub fn get_next_id() -> usize {
      unsafe { COUNTER.fetch_add(1, Ordering::Relaxed) }
  }
}
