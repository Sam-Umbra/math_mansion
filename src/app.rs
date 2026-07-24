use std::fmt;

use crate::math::Problem;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub math_problem: Problem,
    pub menu_state: Menu,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[derive(Debug)]
pub struct Menu {
    pub items: Vec<(MenuItem, bool)>,
}

impl Default for Menu {
    fn default() -> Self {
        let mut items: Vec<(MenuItem, bool)> = Vec::new();

        for item in MenuItem::iter() {
            if item == MenuItem::Start {
                items.push((item, true));
            } else {
                items.push((item, false));
            }
        }

        Self { items }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Start,
    History,
    Help,
    AboutUs,
    Quit,
}

impl MenuItem {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "Start",
            Self::History => "History",
            Self::Help => "Help",
            Self::AboutUs => "About Us",
            Self::Quit => "Quit",
        }
    }

    pub fn iter() -> impl Iterator<Item = MenuItem> {
        [
            MenuItem::Start,
            MenuItem::History,
            MenuItem::Help,
            MenuItem::AboutUs,
            MenuItem::Quit,
        ]
        .iter()
        .copied()
    }
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::Start
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
