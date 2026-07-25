use color_eyre::eyre::Result;
use math_mansion::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();
    let mut terminal = ratatui::init();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
