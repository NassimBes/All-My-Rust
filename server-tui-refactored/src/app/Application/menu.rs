use color_eyre::eyre::Ok;
use ratatui::{
    layout::{self, Rect}, style::{palette::tailwind::SLATE, Color, Modifier, Style}, widgets::{Block, List, ListItem, ListState}, Frame
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app::{Application::App,Application::Screen};


const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug,Default)]
pub struct MenuState{
    items: Vec<&'static str>,
    selected: usize,
}

impl MenuState{
    pub fn new(items: Vec<&'static str>) -> Self{
        Self { items, selected:0 }
    }


    pub fn next(&mut self){
        if self.selected < self.items.len(){
            self.selected += 1;
        }
    }

    pub fn previous(&mut self){
        if self.selected > 0{
            self.selected -= 1;
        }
    }

    pub fn selected_item(&self) -> &str{
        self.items[self.selected]
    }


    pub fn render(&mut self, frame: &mut Frame, state: &MenuState){
        let items = self.items.iter().map(|&i| {
            ListItem::new(i)
            .style(Style::default().fg(Color::Green))
        });

        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));
        
        frame.render_stateful_widget(
            List::new(items)
            .block(
                Block::bordered()
                .title("Menu").title_alignment(layout::Alignment::Center)
            )
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
            frame.area(),
            &mut list_state
        );
    }

}