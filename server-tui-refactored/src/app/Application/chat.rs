

use std::rc::Rc;

use color_eyre::eyre::Ok;
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Paragraph},
    Frame,
};


use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct ChatState{
    items: Vec<&'static str>,
    pub input: String,
    selected: usize,
}

impl Default for ChatState{
    fn default() -> Self {
        Self { 
            items: vec!["Room 1".into(), "Room 2".into(), "Room 3".into(), "Room 4".into()],
            input: String::new(),
            selected: 0 }
    }
}

impl ChatState {
    pub fn next(&mut self) {
        if self.selected  + 1< self.items.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected_item(&self) -> &str {
        self.items[self.selected]
    }
}


pub fn render(frame: &mut Frame, area: Rect, state: &ChatState){
    let items: Vec<ListItem> = state
        .items
        .iter()
        .map(|&i| ListItem::new(i).style(Style::default().fg(Color::Green)))
        .collect();
    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));

    let layout = Layout::new(
        Direction::Horizontal, 
        [Constraint::Percentage(10),Constraint::Percentage(80),Constraint::Percentage(10)],

        )
        .split(frame.area());
    
    render_chatrooms(frame,&layout,items.clone(),&mut list_state);
    render_messageboard(frame,&layout,items.clone(),&mut list_state, state);   
    render_chatters(frame,&layout,items.clone(),&mut list_state);   

     

        

    
}


fn render_chatrooms(frame: &mut Frame, layout: &Rc<[Rect]>,items: Vec<ListItem>, mut list_state: &mut ListState){
    frame.render_stateful_widget(List::new(items)
        .block(
            Block::bordered()
            .title("Rooms").title_alignment(layout::Alignment::Center)
        )
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
        layout[0],  
        &mut list_state
    );       
}


fn render_messageboard(frame: &mut Frame, layout: &Rc<[Rect]>, items: Vec<ListItem>, mut list_state: &mut ListState, state: &ChatState){
    // Message Board
    frame.render_widget(
Paragraph::default()
            .block(
                Block::bordered()
                    .title("Chat Room")
                    .title_alignment(layout::Alignment::Center)
            )
    , layout[1]);

    let middle_chunks = Layout::new(
        Direction::Vertical,
            [
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(layout[1]);

    frame.render_widget(Paragraph::default()
            .block(
                Block::bordered()
                .title("Messages")
                .title_alignment(layout::Alignment::Center)
            )
        ,
        middle_chunks[0]);

    frame.render_widget(Paragraph::new(state.input.as_str())
            .block(
                Block::bordered()
                .title("Input")
                .title_alignment(layout::Alignment::Center)
            )
        ,
        middle_chunks[1]);
}

fn render_chatters(frame: &mut Frame, layout: &Rc<[Rect]>, items: Vec<ListItem>, mut list_state: &mut ListState){
// Connected Users in the Selected Chat 
    frame.render_widget(
Paragraph::default()
            .block(
                Block::bordered()
                    .title("Chatters")
                    .title_alignment(layout::Alignment::Center)
            )
    , layout[2]);
}