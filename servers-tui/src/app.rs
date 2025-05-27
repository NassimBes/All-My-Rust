use color_eyre::{eyre::Ok, Result};
use ratatui::{
        layout::{self, Constraint, Direction, Layout, Rect},
        style::{Color, Style}, 
        widgets::{Block, List, ListItem, Paragraph},
        prelude::{CrosstermBackend},
        Terminal,
        Frame
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::rc::Rc;


#[derive(Debug,Default)]
pub struct App{
    exit: bool
}


struct ChatRooms<'a>{
    frame:  &'a mut Frame<'a>
}

impl<'a> ChatRooms<'a>{
    fn new(f: &'a mut Frame<'a>) -> Self{
        ChatRooms {frame: f}
    }

    fn render(&'a mut self){
        let layout = Layout::new(
        Direction::Horizontal, 
        [Constraint::Percentage(10),Constraint::Percentage(80),Constraint::Percentage(10)],
        )
        .split(self.frame.area());

        Self::render_rooms_block(self.frame,&layout);
        // Self::render_chatroom_block(self.frame,&layout);
        // Self::render_chatter_block(self.frame,&layout);
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
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        , layout[0]);        
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

impl App{
    pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
        let rooms = ChatRooms::new;
        while !self.exit {
            terminal.draw(rooms)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // ANCHOR: handle_key_event fn
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self){
        self.exit = true;
    }

    // fn render_chatrooms(frame: &mut Frame){
    //     let layout = Layout::new(
    //     Direction::Horizontal, 
    //     [Constraint::Percentage(10),Constraint::Percentage(80),Constraint::Percentage(10)],
    //     )
    //     .split(frame.area());

    //     Self::render_rooms_block(frame,&layout);
    //     Self::render_chatroom_block(frame,&layout);
    //     Self::render_chatter_block(frame,&layout);
    // }
    
    // fn render_rooms_block(frame: &mut Frame, layout: &Rc<[Rect]> ){  
    //     let chatrooms = vec!["Total","Total","Total","Total","Total","Total"];
    //     let binding = chatrooms.iter().map(|&f| {
    //         ListItem::new(f)
    //         .style(Style::default().fg(Color::Green))
    //     });

    //     frame.render_widget(List::new(binding)
    //     .block(
    //         Block::bordered()
    //         .title("Rooms").title_alignment(layout::Alignment::Center)
    //     )
    //     .highlight_symbol(">")
    //     .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
    //     , layout[0]);        
    // }
    
    // fn render_chatroom_block(frame: &mut Frame, layout: &Rc<[Rect]>){
    //     frame.render_widget(
    // Paragraph::default()
    //             .block(
    //                 Block::bordered()
    //                     .title("Chat Room")
    //                     .title_alignment(layout::Alignment::Center)
    //             )
    //     , layout[1]);
    // }

    // fn render_chatter_block(frame: &mut Frame, layout: &Rc<[Rect]>){
    //     frame.render_widget(
    // Paragraph::default()
    //             .block(
    //                 Block::bordered()
    //                     .title("Users in Current Chat room")
    //                     .title_alignment(layout::Alignment::Center)
    //             )
    //     , layout[2]);
    // }
}