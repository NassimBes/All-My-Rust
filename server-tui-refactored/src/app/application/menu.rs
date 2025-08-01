// use color_eyre::eyre::Ok;
use ratatui::{
    layout::{self, Rect},
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    widgets::{Block, HighlightSpacing, List, ListItem, ListState},
    Frame,
};


const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct MenuState {
    items: Vec<&'static str>,
    selected: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            items: vec!["Create Server".into(), "Join Server".into()],
            selected: 0,
        }
    }
}

impl MenuState {
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

pub fn render(frame: &mut Frame, area: Rect, state: &MenuState) {
    let items: Vec<ListItem> = state
        .items
        .iter()
        .map(|&i| ListItem::new(i).style(Style::default().fg(Color::Green)))
        .collect();
    

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));

    frame.render_stateful_widget(
        List::new(items)
            .block(
                Block::bordered()
                    .title("Menu")
                    .title_alignment(layout::Alignment::Center),
            )
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always),
        center(area, layout::Constraint::Length(20), layout::Constraint::Length(20)),
        &mut list_state,
    );
}

fn center(area: Rect, horizontal: layout::Constraint, vertical: layout::Constraint) -> Rect {
    let [area] = layout::Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);
    let [area] = layout::Layout::vertical([vertical]).flex(layout::Flex::Center).areas(area);
    area
}
