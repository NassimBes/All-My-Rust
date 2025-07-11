use ratatui::{
    widgets::ListState
};



#[derive(Debug,Default)]
pub struct SelectorState{
    pub list_state: ListState
}

impl SelectorState {
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

