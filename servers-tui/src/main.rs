
use color_eyre::{eyre::Ok, Result};
use ratatui::{
    prelude::CrosstermBackend,
    Terminal
};


mod app {
    use super::*;
    #[allow(unused_imports)]
    use ratatui::{
        layout::{self, Constraint, Direction, Layout, Rect},
        style::{Color, Style}, 
        widgets::{Block, Borders, List, ListItem, Paragraph},
        Frame
    };

    use crossterm::{event::{self, Event, KeyCode, KeyEvent}, style};
    use std::rc::Rc;


    pub fn run(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
        loop {
            terminal.draw(render)?;
            if matches!(event::read()?, Event::Key(KeyEvent{
                    code: KeyCode::Esc,
                    ..}
                )
            ){ break Ok(()) }
        }
    }

    fn render(frame: &mut Frame){
        let layout = Layout::new(
        Direction::Horizontal, 
        [Constraint::Percentage(15),Constraint::Percentage(70),Constraint::Percentage(15)],
        )
        .split(frame.area());

        render_rooms_block(frame,&layout);
        render_chatroom_block(frame,&layout);
        render_chatter_block(frame,&layout);
    }
    
    fn render_rooms_block(frame: &mut Frame, layout: &Rc<[Rect]> ){  
        let chatrooms = vec!["Total","Total","Total","Total","Total","Total"];
        let binding = chatrooms.iter().map(|&f| {
            ListItem::new(f)
            .style(Style::default().fg(Color::Green))
        });

        frame.render_widget(List::new(binding)
        .block(
            Block::bordered()
            .title("Rooms").title_alignment(layout::Alignment::Center)
        )
        .highlight_symbol(">>>")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        , layout[0]);
        // frame.render_widget(
        // List::new([
        //     Text::styled("Total", Style::default().fg(Color::Red)),
        //     Text::styled("Total", Style::default().fg(Color::Red)),
        //     Text::styled("Total", Style::default().fg(Color::Red)),
        //     Text::styled("Total", Style::default().fg(Color::Red)),
        //     Text::styled("Total", Style::default().fg(Color::Red)),
        // ]).block(
        //     Block::bordered()
        //     .title("Rooms")
        //     .title_alignment(layout::Alignment::Center)
        // ),
        // layout[0]);
        
    }
    fn render_chatroom_block(frame: &mut Frame, layout: &Rc<[Rect]>){
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Chat Room")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[1]);
    }

    fn render_chatter_block(frame: &mut Frame, layout: &Rc<[Rect]>){
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Users in Current Chat room")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[2]);
    }
    
    
}

fn main() -> Result<()>{
   color_eyre::install()?;
   let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();
   let result = app::run(terminal);

   ratatui::restore();
   result
}


