use std::fmt;

use ratatui::{layout::Rect, widgets::ListState};

#[derive(Debug)]
pub struct Menu {
    pub menu_options: Vec<MenuOptions>,
    pub state: ListState,
    pub area: Rect,
}

impl Default for Menu {
    fn default() -> Self {
        let mut menu_options: Vec<MenuOptions> = Vec::new();
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        for option in MenuOptions::iter() {
            menu_options.push(option);
        }

        Self {
            menu_options: menu_options,
            state: menu_state,
            area: Rect::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuOptions {
    Start,
    History,
    Help,
    AboutUs,
    Quit,
}

impl MenuOptions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "Start",
            Self::History => "History",
            Self::Help => "Help",
            Self::AboutUs => "About Us",
            Self::Quit => "Quit",
        }
    }

    pub fn iter() -> impl Iterator<Item = MenuOptions> {
        [
            MenuOptions::Start,
            MenuOptions::History,
            MenuOptions::Help,
            MenuOptions::AboutUs,
            MenuOptions::Quit,
        ]
        .iter()
        .copied()
    }
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self::Start
    }
}

impl fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
