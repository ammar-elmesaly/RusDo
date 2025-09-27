use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Direction}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, List, ListItem, ListState, Padding}, Frame
};
use super::Menu;

pub fn draw_menu(frame: &mut Frame, menu: &Menu) {
    let area = centered_rect(60, 100, frame.area());
    let items: Vec<ListItem> = menu.items.iter().map(|item | ListItem::new(Line::from(item.content).alignment(Alignment::Center))).collect();

    let block_style = Block::bordered()
    .title("Welcome to RusDo!")
    .title_alignment(Alignment::Center)
    .padding(Padding::symmetric(2, 1));

    let list: List = List::new(items).block(block_style)
    .magenta()
    .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White));

    let mut state = ListState::default();
    state.select(Some(menu.selected));

    frame.render_stateful_widget(list, area, &mut state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let vertical_chunk = popup_layout[1];

    let horizontal_layout = Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .direction(Direction::Horizontal)
        .split(vertical_chunk);

    horizontal_layout[1]
}
