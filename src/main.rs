use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use math_mansion::app::App;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();

    let terminal = ratatui::init();
    let result = run(terminal, &mut app);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &App) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let menu_items: Vec<String> = app_state
        .menu_state
        .items
        .iter()
        .map(|x| x.0.to_string())
        .collect();

    List::new(menu_items.iter().map(|x| ListItem::from(x.to_string())))
        .render(inner_area, frame.buffer_mut());

    // Paragraph::new(format!("{menu_items:?}")).render(frame.area(), frame.buffer_mut());
}
