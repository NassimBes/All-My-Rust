
use color_eyre::{eyre::Ok, Result};
use crossterm::{event::{self, Event}, terminal};
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, widgets::{Block, Paragraph}, DefaultTerminal, Frame, Terminal
    
};

mod app {
    use std::rc::Rc;

    use super::*;
    
    pub fn run(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> std::result::Result<(), color_eyre::eyre::Error>{
        loop {
            terminal.draw(render)?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(())
            }
        }
    }

    fn render(frame: &mut Frame){
        let layout = Layout::new(
        Direction::Horizontal, 
        [Constraint::Percentage(15),Constraint::Percentage(70),Constraint::Percentage(15)],
        )
        .split(frame.area());

        render_left_block(frame,&layout);
        render_middle_block(frame,&layout);
        render_right_block(frame,&layout);
        
        fn render_left_block(frame: &mut Frame, layout: &Rc<[Rect]> ){
                frame.render_widget(Paragraph::default()
                .block(Block::bordered()
                .title("Left Block")
                .title_alignment(layout::Alignment::Center)
            )
                , layout[0]);
        }
        fn render_middle_block(frame: &mut Frame, layout: &Rc<[Rect]>){
            frame.render_widget(Paragraph::default()
            .block(Block::bordered()
            .title("Chat Room")
            .title_alignment(layout::Alignment::Center)
        )
            , layout[1]);
        }

        fn render_right_block(frame: &mut Frame, layout: &Rc<[Rect]>){
            frame.render_widget(Paragraph::default()
            .block(Block::bordered()
            .title("Users in Current Chat room").title_alignment(layout::Alignment::Center).
        )
            , layout[2]);
        }
    }
    
}

fn main() -> Result<()>{
   color_eyre::install()?;
   let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();
   let result = app::run(terminal);

   ratatui::restore();
   result
}


