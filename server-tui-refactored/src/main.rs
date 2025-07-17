use color_eyre::{Result};
use ratatui::{
    prelude::CrosstermBackend,
    Terminal
};

mod app;
use app::application::App;


// const MIN_WIDTH: u16 = 80;
// const MIN_HEIGHT: u16 = 24;

fn main() -> Result<()>{
    color_eyre::install()?;
    let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();

    let result = App::default().run(terminal);


    ratatui::restore();
    result
}