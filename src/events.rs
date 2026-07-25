use crossterm::event::{self, KeyEvent};
use ratatui::widgets::ListState;

use crate::app::App;

pub struct EventHandler;

impl EventHandler {
    pub fn handle_key(app: &mut App, key: KeyEvent) {
        match key.code {
            event::KeyCode::Esc => app.quit(),
            event::KeyCode::Char(char) => match char {
                'j' => Self::next(&mut app.menu.state, &app.menu.menu_options),
                'k' => Self::previous(&mut app.menu.state, &app.menu.menu_options),
                _ => {}
            },
            event::KeyCode::Up => {
                Self::previous(&mut app.menu.state, &app.menu.menu_options);
            }
            event::KeyCode::Down => {
                Self::next(&mut app.menu.state, &app.menu.menu_options);
            }
            _ => {}
        }
    }

    fn next<T>(list: &mut ListState, items: &Vec<T>) {
        let i: usize = match list.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        list.select(Some(i));
    }

    fn previous<T>(list: &mut ListState, items: &Vec<T>) {
        let i: usize = match list.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        list.select(Some(i));
    }
}
