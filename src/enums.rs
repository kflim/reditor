pub mod enums {
  pub enum FindBarState {
    Finding,
    Focused,
    NotFocused,
  }

  impl Clone for FindBarState {
    fn clone(&self) -> Self {
      match self {
        FindBarState::Finding => FindBarState::Finding,
        FindBarState::Focused => FindBarState::Focused,
        FindBarState::NotFocused => FindBarState::NotFocused,
      }
    }
  }

  pub enum ReplaceBarState {
    Focused,
    NotFocused,
    Replacing,
  }

  impl Clone for ReplaceBarState {
    fn clone(&self) -> Self {
      match self {
        ReplaceBarState::Replacing => ReplaceBarState::Replacing,
        ReplaceBarState::Focused => ReplaceBarState::Focused,
        ReplaceBarState::NotFocused => ReplaceBarState::NotFocused,
      }
    }
  }

  pub enum GoToState {
    GoingTo,
    Focused,
    NotFocused,
  }

  impl Clone for GoToState {
    fn clone(&self) -> Self {
      match self {
        GoToState::GoingTo => GoToState::GoingTo,
        GoToState::Focused => GoToState::Focused,
        GoToState::NotFocused => GoToState::NotFocused,
      }
    }
  }

  impl std::fmt::Display for FindBarState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FindBarState::Focused => write!(f, "Focused"),
            FindBarState::NotFocused => write!(f, "Not Focused"),
            FindBarState::Finding => write!(f, "Finding"),
        }
    }
  }
}
