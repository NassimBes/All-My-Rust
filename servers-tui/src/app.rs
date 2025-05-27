use color_eyre::{eyre::Ok, Result};
use ratatui::{
        layout::{self, Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, style::{Color, Style}, widgets::{Block, List, ListItem, ListState, Paragraph}, Frame, Terminal
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::rc::Rc;


const MIN_WIDTH: u16 = 80;
const MIN_HEIGHT: u16 = 24;

#[derive(Debug,Default)]
pub struct ChatRoomState{
    list_state: ListState
}

impl ChatRoomState {
    pub fn next(&mut self){
        let selected = self.selected().unwrap();
        self.list_state.select(Some(selected+1));
    }
    pub fn previous(&mut self){
        let selected = self.selected().unwrap();
        if selected != 0{
            self.list_state.select(Some(selected-1));
        }
    }
    pub fn selected(&mut self) -> Option<usize>{
        self.list_state.selected()
    }

    
}


#[derive(Debug,Default)]
pub struct ChatRooms;

impl ChatRooms{
    pub fn render(frame: &mut Frame,states: &mut ChatRoomState){
        if frame.area().width > MIN_WIDTH && frame.area().height > MIN_HEIGHT {
            
            let layout = Layout::new(
            Direction::Horizontal, 
            [Constraint::Percentage(10),Constraint::Percentage(80),Constraint::Percentage(10)],
        
            )
            .split(frame.area());

            Self::render_rooms_block(frame,&layout, states);
            Self::render_chatroom_block(frame,&layout);
            Self::render_chatter_block(frame,&layout);
        }
        else{
            let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(frame.area());
            Self::render_chat_only(frame,&layout);
        }
    }

    fn render_chat_only(frame: &mut Frame, layout: &Rc<[Rect]>){
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Chat Room")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[0]);
    }

    // ListState
    // render_stateful_widget
    fn render_rooms_block(frame: &mut Frame, layout: &Rc<[Rect]>, states : &mut ChatRoomState){  
        let chatrooms = vec!["Total","Total","Total","Total","Total"];
        
        let binding = chatrooms.iter().map(|&f| {
            ListItem::new(f)
            .style(Style::default().fg(Color::Green))
        });
        
        
        
        frame.render_stateful_widget(List::new(binding)
        .block(
            Block::bordered()
            .title("Rooms").title_alignment(layout::Alignment::Center)
        )
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
         layout[0],
        &mut states.list_state
        );        
        
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


#[derive(Debug,Default)]
pub struct App{
    exit: bool,
    chatroomstate: ChatRoomState
}

impl App{
    pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
        self.chatroomstate.list_state.select(Some(0));
        while !self.exit {
            terminal.draw(|f| ChatRooms::render(f, &mut self.chatroomstate))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };
        Ok(())
    }

    // ANCHOR: handle_key_event fn
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Down => self.chatroomstate.next(),
            KeyCode::Up => self.chatroomstate.previous(),
            _ => {}
        }
    }

    fn exit(&mut self){
        self.exit = true;
    }

    
}