use color_eyre::{eyre::Ok, Result};
use ratatui::{
    prelude::CrosstermBackend, Terminal
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};



pub mod chatrooms;

use chatrooms::{
    ChatRooms,
    chatroomstate::ChatRoomState
};

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