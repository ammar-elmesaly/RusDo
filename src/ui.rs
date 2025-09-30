use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Block, List, ListItem, ListState, Padding, Paragraph, Wrap}, Frame
};
use super::Menu;
use crate::task::{Task, TaskList};

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

pub fn draw_view(frame: &mut Frame, task_list: &TaskList) {
    let area = centered_rect(60, 100, frame.area());
    let outer_block = Block::bordered()
        .title("Task View")
        .title_alignment(Alignment::Center)
        .light_blue();

    // Renders large border with a title
    frame.render_widget(outer_block, area);

    let area = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(90)
    ]).split(area);   // Split area up

    let info_message = Paragraph::new("Press D to delete a task or Enter to view.")
    .centered()
    .block(Block::default().padding(Padding::top(2)).bold())
    .light_blue()
    .wrap(Wrap { trim: true });

    frame.render_widget(info_message, area[0]);
    

    let items: Vec<ListItem> = task_list.tasks.iter().enumerate().map(|(index, task) | {
        let span;

        if task.completed {
            span = Span::default().content(format_task(index, task)).crossed_out();
        } else {
            span = Span::default().content(format_task(index, task));
        }

        ListItem::new(span)
    }).collect();

    let block_style = Block::new()
    .padding(Padding::new(frame.area().width / 5, 1, 1, 1)); 

    let list: List = List::new(items).block(block_style)
    .light_blue()
    .highlight_symbol(">> ")
    .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::White))
    .scroll_padding(3);
    
    let mut state = ListState::default();
    state.select(Some(task_list.selected));

    frame.render_stateful_widget(list, area[1], &mut state);
}

pub fn draw_view_task(frame: &mut Frame, task: &Task, selected: usize) {
    
    let area = centered_rect(60, 100, frame.area());
    let outer_block = Block::bordered()
        .title("Task View")
        .title_alignment(Alignment::Center);

    // Renders large border with a title
    frame.render_widget(outer_block, area);

    // block styles
    let title_block_style = Block::default().padding(Padding::top(3));
    let desc_block_style = Block::default().padding(Padding::horizontal(frame.area().width / 10));
    
    let area = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(50),
        Constraint::Percentage(15),
        Constraint::Percentage(15)
    ]).split(area);   // Split area up


    let task_title = Paragraph::new(task.title.to_string()).centered().block(title_block_style).style(Style::default().bold());
    
    frame.render_widget(task_title, area[0]);  // Render task title

    if let Some(task_desc) = &task.desc {
        let task_desc = Paragraph::new(format!("Description: {}", task_desc.to_string()))
        .centered()
        .wrap(Wrap {trim: true})
        .block(desc_block_style);

        frame.render_widget(task_desc, area[1]);  // Render task description (if it exists)
    }

    let confirm_delete_message = Line::from("Mark this task as done?").centered().style(Style::default().bold());
    frame.render_widget(confirm_delete_message, area[2]);

    let button_areas = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33)
    ]).split(area[3]);


    // Buttons with highlight based on `selected`
    let yes_style = if selected == 0 {
        Style::default().bg(Color::Yellow).fg(Color::White).bold()
    } else {
        Style::default()
    };

    let no_style = if selected == 1 {
        Style::default().bg(Color::Yellow).fg(Color::White).bold()
    } else {
        Style::default()
    };

    let yes = Paragraph::new(Span::styled("[ Yes ]", yes_style)).right_aligned();
    let no  = Paragraph::new(Span::styled("[ No ]",  no_style)).left_aligned();

    frame.render_widget(yes, button_areas[0]);
    frame.render_widget(no,  button_areas[2]);

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

fn format_task(index: usize, task: &Task) -> String {
    format!("{}. {}", index + 1, task.title)
}