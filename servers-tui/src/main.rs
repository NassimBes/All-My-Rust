use color_eyre::{Result};
use ratatui::{
    prelude::CrosstermBackend,
    Terminal
};

pub mod app;
use app::App;




fn main() -> Result<()>{
   color_eyre::install()?;
   let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();
   let result = App::default().run(terminal);

   ratatui::restore();
   result
}


