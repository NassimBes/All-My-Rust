pub mod application{
    use color_eyre::{eyre::Ok, Result};
    use ratatui::{
        prelude::CrosstermBackend,
        Terminal,
        Frame
    };

    #[allow(unused_imports)]
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

    mod menu;
    mod chat;


    use menu::{MenuState, render as render_menu};
    use chat::{ChatState, render as render_chat};

    
    
    #[derive(Debug,PartialEq,Eq,Clone,Copy)]
    #[allow(dead_code)]
    pub enum Screen {
        Menu,
        ChatSelector,
        ChatRoom    
    }

    #[derive(Debug)]
    pub struct App{
        pub screen: Screen,
        chat_state: ChatState,
        menu_state: MenuState,
        // selector_state: SelectorState,
        exit: bool,
    }

    impl Default for App {
        fn default() -> Self {
            Self { 
                screen: Screen::Menu, 
                menu_state: MenuState::default() , 
                chat_state: ChatState::default(),
                exit: false, 
            }
        }
    }
    impl App{
        pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
            self.screen = Screen::Menu;
            while !self.exit {
                terminal.draw(|frame| self.render(frame))?;
                self.handle_event()?;
            }

            Ok(())
        }

        fn render(&mut self, frame: &mut Frame){
            match self.screen {
                    Screen::Menu => {
                        render_menu(frame, frame.area(), &self.menu_state);
                    },
                    Screen::ChatRoom => {
                        render_chat(frame, frame.area(), &self.chat_state);
                    },
                    Screen::ChatSelector => {

                    }
            }
        }

        fn handle_event(&mut self) -> Result<()>{
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = crossterm::event::read()?{
                    match self.screen {
                        Screen::Menu => {
                            self.handle_menu_input(key.code);
                        },
                        Screen::ChatRoom => {
                            self.handle_chat_input(key.code);
                        },
                        Screen::ChatSelector => {

                        }
                    }
                }
            }
            Ok(())
        }

        fn handle_menu_input(&mut self, code: KeyCode){
            match code {
                KeyCode::Up => self.menu_state.previous(),
                KeyCode::Down => self.menu_state.next(),
                KeyCode::Enter => {
                    let selected = self.menu_state.selected_item();
                    match selected {
                        "Create Server" => self.screen = Screen::ChatRoom, // or Screen::CreateServer
                        "Join Server" => self.screen = Screen::ChatRoom, // or Screen::JoinServer
                        _ => {}
                    }
                }
                KeyCode::Esc => self.exit = true,
                _ => {}
            }
        }

        fn handle_chat_input(&mut self, code: KeyCode){
            match code {
                KeyCode::Up => self.chat_state.previous(),
                KeyCode::Down => self.chat_state.next(),
                KeyCode::Char(c) => self.chat_state.input.push(c),
                KeyCode::Backspace => { self.chat_state.input.pop(); },
                KeyCode::Enter => {
                // Optional: pretend to "send" message and clear buffer
                    self.chat_state.input.clear();
                }
                KeyCode::Esc => self.screen = Screen::Menu,
                _ => {}
            }
        }
        
    }
}