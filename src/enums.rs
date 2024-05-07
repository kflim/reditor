pub mod enums {
  pub enum FindBarState {
    Closed,
    Focused,
    NotFocused,
    Finding,
  }

  pub enum ReplaceBarState {
      Closed,
      Focused,
      NotFocused,
      Replacing,
  }

  pub enum GoToState {
      Closed,
      Focused,
      NotFocused,
      GoingTo,
  }

  impl std::fmt::Display for FindBarState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FindBarState::Closed => write!(f, "Closed"),
            FindBarState::Focused => write!(f, "Focused"),
            FindBarState::NotFocused => write!(f, "Not Focused"),
            FindBarState::Finding => write!(f, "Finding"),
        }
    }
  }
}
