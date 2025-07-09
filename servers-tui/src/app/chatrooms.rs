use ratatui::{
        layout::{self, Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, List, ListItem, Paragraph}, Frame
};
use std::rc::Rc;

pub mod selectorstate;

use selectorstate::SelectorState;



#[derive(Debug,Default)]
pub struct ChatRooms;

impl ChatRooms{
    pub fn render(frame: &mut Frame,states: &mut SelectorState, min_width: u16, min_height: u16){
        if frame.area().width > min_width && frame.area().height > min_height {
            
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
    fn render_rooms_block(frame: &mut Frame, layout: &Rc<[Rect]>, states : &mut SelectorState){  
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
                        .title("Chatters")
                        .title_alignment(layout::Alignment::Center)
                )
        , layout[2]);
    }

    


    
}
