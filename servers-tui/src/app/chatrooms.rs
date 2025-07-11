use ratatui::{
        layout::{self, Constraint, Direction, Layout, Rect}, style::{palette::tailwind::SLATE, Color, Modifier, Style}, widgets::{Block, List, ListItem, Paragraph}, Frame
};
use std::rc::Rc;

pub mod selectorstate;

use selectorstate::SelectorState;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug,Default)]
pub struct ChatRooms;

impl ChatRooms{
    pub fn render(&mut self, frame: &mut Frame,states: &mut SelectorState, min_width: u16, min_height: u16){
        if frame.area().width > min_width && frame.area().height > min_height {
            
            let layout = Layout::new(
            Direction::Horizontal, 
            [Constraint::Percentage(10),Constraint::Percentage(80),Constraint::Percentage(10)],
        
            )
            .split(frame.area());

            self.render_chat(frame,&layout, states);

        }
        else{
            let [layout] = Layout::horizontal([Constraint::Percentage(100)]).areas(frame.area());
            self.render_chat_only(frame,&layout);
        }
    }

    fn render_chat_only(&mut self, frame: &mut Frame, area: &Rect){
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Chat Room")
                        .title_alignment(layout::Alignment::Center)
                )
        , *area);
    }


    fn render_chat(&mut self, frame: &mut Frame,layout: &Rc<[Rect]>,  states : &mut SelectorState){
        let chatrooms = vec!["Total","Total","Total","Total","Total"];
        
        let binding = chatrooms.iter().map(|&f| {
            ListItem::new(f)
            .style(Style::default().fg(Color::Green))
        });
        
        // ChatRoom Selector
        frame.render_stateful_widget(List::new(binding)
        .block(
            Block::bordered()
            .title("Rooms").title_alignment(layout::Alignment::Center)
        )
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
        layout[0],
        &mut states.list_state
        );        

        // Message Board
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Chat Room")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[1]);

        // Contenect Users in the Selected Chat 
        frame.render_widget(
    Paragraph::default()
                .block(
                    Block::bordered()
                        .title("Chatters")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[2]);
    }
    
    
}
