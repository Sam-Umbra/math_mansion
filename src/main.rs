use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use math_mansion::app::App;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
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
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => {
                        app_state.menu.menu_state.select_next();
                    }
                    'k' => {
                        app_state.menu.menu_state.select_previous();
                    }
                    _ => {}
                },
                event::KeyCode::Up => {
                    app_state.menu.menu_state.select_previous();
                }
                event::KeyCode::Down => {
                    app_state.menu.menu_state.select_next();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut App) {
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

    let menu_options: Vec<String> = app_state
        .menu
        .menu_options
        .iter()
        .map(|x| x.to_string())
        .collect();

    let menu = List::new(menu_options.iter().map(|x| ListItem::from(x.to_string())))
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">");

    frame.render_stateful_widget(menu, inner_area, &mut app_state.menu.menu_state);

    // Paragraph::new(format!("{menu_items:?}")).render(frame.area(), frame.buffer_mut());
}
