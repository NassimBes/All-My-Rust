use color_eyre::{eyre::Ok, Result};
use ratatui::{
    prelude::CrosstermBackend,
    Terminal
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub mod chatrooms;

use chatrooms::{
    ChatRooms,
    selectorstate::SelectorState
};


#[derive(Debug,Default)]
pub struct App{
    exit: bool,
    selectorstate: SelectorState
}

impl App{
    #[allow(unused_variables)]
    pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>, min_width:u16, min_height: u16) -> Result<()>{
        self.selectorstate.list_state.select_first();
        let mut chatroom = ChatRooms::default();
        while !self.exit {
            terminal.draw(|f| chatroom.render(f, &mut self.selectorstate, min_width, min_height))?;
            // terminal.draw(|f| Menu::render(f, &mut self.selectorstate))?;
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
            KeyCode::Down => self.selectorstate.next(),
            KeyCode::Up => self.selectorstate.previous(),
            _ => {}
        }
    }

    fn exit(&mut self){
        self.exit = true;
        
    }
    
}