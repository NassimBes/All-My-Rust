use ratatui::{
        layout::{self, Constraint, Flex, Layout, Rect}, style::{palette::tailwind::SLATE, Color, Modifier, Style}, widgets::{Block, List, ListItem}, Frame
};



const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);


use crate::app::chatrooms::selectorstate::SelectorState;




const MENUITEMS: [&str; 3] = ["Join Server","Create Server","Exit"];


#[derive(Debug,Default)]
pub struct Menu;


impl Menu{
    pub fn render(frame: &mut Frame, states: &mut SelectorState){
        let area = Self::center(
            frame.area(),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        );
        Self::render_menu(frame,&area,states);
    }

    fn render_menu(frame: &mut Frame,area: &Rect, states: &mut SelectorState){
        // let menu_items = vec!["Join Server","Create Server","Exit"];
        
        let binding = MENUITEMS.iter().map(|&f| {
            ListItem::new(f)
            .style(Style::default().fg(Color::Green))
        });
        
        
        
        frame.render_stateful_widget(List::new(binding)
            .block(
                Block::bordered()
                .title("Rooms").title_alignment(layout::Alignment::Center)
            )
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
            *area,
            &mut states.list_state
            );
    }


    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        area
    }


}