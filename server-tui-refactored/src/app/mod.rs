pub mod Application{
    use color_eyre::{eyre::Ok, Result};
    use ratatui::{
        prelude::CrosstermBackend,
        Terminal,
        Frame
    };
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

    mod menu;
    use menu::MenuState;
    

    
    
    #[derive(Debug)]
    pub enum Screen {
        Menu,
        ChatSelector,
        ChatRoom    
    }

    impl Default for Screen {
        fn default() -> Self {
            Screen::Menu
        }
    }

    #[derive(Debug,Default)]
    pub struct App{
        pub screen: Screen,
        // chat_state: ChatState,
        menu_state: MenuState,
        // selector_state: SelectorState,
        exit: bool,
    }

    impl App{
        pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
            while !self.exit {
                terminal.draw(|frame| self.render(frame))?;
                self.handle_event()?;
            }

            Ok(())
        }

        fn handle_event(&mut self) -> Result<()>{
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = crossterm::event::read()?{
                    match self.screen {
                        Screen::Menu => {
                            // self.menu_state.handle_events();
                        },
                        Screen::ChatRoom => {

                        },
                        Screen::ChatSelector => {

                        }
                    }
                }
            }
            Ok(())
        }

        fn render(&mut self, frame: &mut Frame){
            match self.screen {
                    Screen::Menu => {
                        self.menu_state.render(frame, &MenuState::new(["C","J"].to_vec()));
                    },
                    Screen::ChatRoom => {

                    },
                    Screen::ChatSelector => {

                    }
            }
        }

        fn exit(&mut self){
            self.exit = true;
        }
    }
}