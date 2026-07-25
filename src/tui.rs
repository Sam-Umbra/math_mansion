use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, List, ListItem, Widget},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let (menu_per, aside_per) = if frame.area().width > 150 {
        (30, 45)
    } else if frame.area().width < 85 {
        (30, 0)
    } else {
        (30, 30)
    };
    let [menu_area, main_area, aside] = Layout::horizontal([
        Constraint::Percentage(menu_per),
        Constraint::Fill(1),
        Constraint::Max(aside_per),
    ])
    .spacing(Spacing::Overlap(1))
    .areas(frame.area());

    // let [menu_area, main_area, aside] = Layout::horizontal([Constraint::Fill(1); 3])
    //     .spacing(Spacing::Overlap(1))
    //     .areas(frame.area());

    let [chart, details] = Layout::vertical([Constraint::Fill(1); 2])
        .spacing(Spacing::Overlap(1))
        .areas(aside);

    let areas = vec![menu_area, main_area, aside, chart, details];
    render_borders(areas, frame);

    let [menu_inner] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(menu_area);

    render_menu(app, frame, menu_inner);
}

fn render_menu(app: &mut App, frame: &mut Frame, menu_area: Rect) {
    app.menu.area = menu_area;

    let menu_options: Vec<String> = app
        .menu
        .menu_options
        .iter()
        .map(|x| x.to_string())
        .collect();

    let menu = List::new(menu_options.iter().map(|x| ListItem::from(x.to_string())))
        .highlight_style(Style::default().fg(Color::Blue))
        .highlight_symbol(">")
        .fg(Color::Cyan);

    frame.render_stateful_widget(menu, menu_area, &mut app.menu.state);
}

fn render_borders(areas: Vec<Rect>, frame: &mut Frame) {
    for area in areas {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::Cyan)
            .merge_borders(MergeStrategy::Fuzzy)
            .render(area, frame.buffer_mut());
    }
}
