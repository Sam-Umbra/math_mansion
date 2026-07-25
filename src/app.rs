use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

use crate::{components::menu::Menu, events::EventHandler, math::Problem, tui};

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub math_problem: Problem,
    pub menu: Menu,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| tui::render(self, frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                EventHandler::handle_key(self, key);
            }
        }

        Ok(())
    }
}
