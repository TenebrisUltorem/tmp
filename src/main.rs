mod state;
mod components;
mod app;

use std::io::Error;

use app::App;

fn main() -> Result<(), Error> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
