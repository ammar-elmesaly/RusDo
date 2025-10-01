use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Padding, Paragraph, Wrap},
    Frame
};
use super::Menu;
use crate::{add_task::SelectedInput, task::{Task, TaskList}};

pub fn draw_menu(frame: &mut Frame, menu: &Menu, show_message: bool) {

    let outer_area = centered_rect(60, 50, frame.area());

    let outer_block = Block::bordered()
        .title("Welcome to RusDo!")
        .title_alignment(Alignment::Center)
        .light_magenta();

    // Renders large border with a title
    frame.render_widget(outer_block, outer_area);

    let area = Layout::vertical([
        Constraint::Percentage(80),
        Constraint::Percentage(20)
    ])
    .split(outer_area);

    let items: Vec<ListItem> = menu.items.iter().map(|item | ListItem::new(Line::from(item.content).centered()))
    .collect();

    let block_style = Block::default()
    .padding(Padding::symmetric(2, 3));

    let list: List = List::new(items).block(block_style)
    .magenta()
    .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White));

    let mut state = ListState::default();
    state.select(Some(menu.selected));

    frame.render_stateful_widget(list, area[0], &mut state);

    if !show_message { return; }

    let message = Paragraph::new("There is no available tasks, consider adding some!")
    .centered()
    .blue()
    .bold()
    .wrap(Wrap { trim: true })
    .block(Block::bordered());

    frame.render_widget(message, area[1]);


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
    .padding(Padding::new(frame.area().width / 5, 1, 1, 2)); 

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


    let task_title = Paragraph::new(task.title.to_string()).centered().block(title_block_style).bold();
    
    frame.render_widget(task_title, area[0]);  // Render task title

    if let Some(task_desc) = &task.desc {
        let task_desc = Paragraph::new(format!("Description: {}", task_desc.to_string()))
        .centered()
        .wrap(Wrap {trim: true})
        .block(desc_block_style);

        frame.render_widget(task_desc, area[1]);  // Render task description (if it exists)
    }

    let confirm_check_message = Line::from("Mark this task as done?").centered().style(Style::default().bold());
    frame.render_widget(confirm_check_message, area[2]);

    let button_areas = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40)
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

pub fn draw_delete_task(frame: &mut Frame, task: &Task, selected: usize) {
    let area = centered_rect(60, 40, frame.area());
    let outer_block = Block::bordered()
        .title("Delete Task")
        .title_alignment(Alignment::Center)
        .red();

    // Renders large border with a title
    frame.render_widget(outer_block, area);

    // block styles
    let title_block_style = Block::default().padding(Padding::top(3));
    
    let area = Layout::vertical([
        Constraint::Percentage(60),
        Constraint::Percentage(40)
    ]).split(area);   // Split area up

    let confirm_delete_message = Paragraph::new(format!("Are you sure to delete this task ({})?", task.title.to_string()))
    .centered()
    .block(title_block_style)
    .bold();
    
    frame.render_widget(confirm_delete_message, area[0]);  // Render task title

        let button_areas = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40)
    ]).split(area[1]);


    // Buttons with highlight based on `selected`
    let yes_style = if selected == 0 {
        Style::default().bg(Color::LightRed).fg(Color::White).bold()
    } else {
        Style::default()
    };

    let no_style = if selected == 1 {
        Style::default().bg(Color::LightRed).fg(Color::White).bold()
    } else {
        Style::default()
    };

    let yes = Paragraph::new(Span::styled("[ Yes ]", yes_style)).right_aligned();
    let no  = Paragraph::new(Span::styled("[ No ]",  no_style)).left_aligned();

    frame.render_widget(yes, button_areas[0]);
    frame.render_widget(no,  button_areas[2]);


}

pub fn draw_about(frame: &mut Frame) {

    let outer_area = centered_rect(60, 80, frame.area());

    let outer_block = Block::bordered()
        .title("About")
        .title_alignment(Alignment::Center)
        .light_green();

    // Renders large border with a title
    frame.render_widget(outer_block, outer_area);

    let area = Layout::vertical([
        Constraint::Percentage(16),
        Constraint::Percentage(50),
        Constraint::Percentage(16),
        Constraint::Percentage(16)
    ]).split(outer_area);

    let about = vec![
        Paragraph::new("This cli to-do list is powered by Rust.").centered().light_green(),
        Paragraph::new("
      @ @@@@ @      
    @@@@@@@@@@@@    
  @@@          @@@  
 @@@@@@@@@@@@@@ @@@ 
@@@@ @@@    @@@ @@@@
@@@  @@@@@@@@@   @@@
@@@  @@@   @@@@ @@@@
 @@@@@@@@@  @@@@@@@ 
  @@@@@      @@@@@  
    @@@@@@@@@@@@    
      @ @@@@ @      
        ").centered().light_green().bold(),
        Paragraph::new("Made by Ammar Elmesaly (release 0.1.0)").centered().light_green(),
        Paragraph::new("Github link: https://www.github.com/ammar-elmesaly").centered().light_green()
    ];

    frame.render_widget(&about[0], center_vertical(area[0],1));
    frame.render_widget(&about[1], area[1]);
    frame.render_widget(&about[2], center_vertical(area[2],1));
    frame.render_widget(&about[3], area[3]);
}

pub fn draw_add_task(frame: &mut Frame, title_text: &str, desc_text: &str, selected_input: &SelectedInput) {
    let outer_area = centered_rect(60, 50, frame.area());

    let outer_block = Block::bordered()
        .title("Add a task")
        .title_alignment(Alignment::Center)
        .yellow();

    // Renders large border with a title
    frame.render_widget(outer_block, outer_area);

    let area = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(75)
    ])
    .split(outer_area);

    let title_style;
    let desc_style;

    match selected_input {
        SelectedInput::Title => {
            title_style = Color::LightYellow.into();
            desc_style = Style::default();
        }
        SelectedInput::Desc => {
            title_style = Style::default();
            desc_style = Color::LightYellow.into();
        } 
    };

    let task_title = Paragraph::new(title_text)
        .block(Block::bordered().title("Task Title"))
        .style(title_style);

    let task_desc = Paragraph::new(desc_text)
        .block(Block::bordered().title("Task Description"))
        .style(desc_style)
        .wrap(Wrap { trim: true });

    frame.render_widget(task_title, area[0]);
    frame.render_widget(task_desc, area[1]);
    
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

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

fn format_task(index: usize, task: &Task) -> String {
    format!("{}. {}", index + 1, task.title)
}