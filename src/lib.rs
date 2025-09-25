use std::io;

use ratatui::{
    layout::{Alignment, Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, symbols::block, text::{Line, Span, Text}, widgets::{Block, List, ListItem, ListState, Padding, Paragraph}, Frame
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub struct Menu {
    pub items: Vec<&'static str>,
    pub selected: usize
}

fn draw(frame: &mut Frame, menu: &Menu) {
    let canvas = Layout::vertical([Constraint::Min(1)]).split(frame.area());
    let items: Vec<ListItem> = menu.items.iter().map(|item | ListItem::new(*item)).collect();

    let block_style = Block::bordered()
    .title("Choose an item")
    .title_alignment(Alignment::Center)
    .padding(Padding::symmetric(2, 1));

    let list: List = List::new(items).block(block_style)
    .light_blue()
    .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White))
    .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(menu.selected));

    frame.render_stateful_widget(list, canvas[0], &mut state);
}

fn handle_events(menu: &mut Menu) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up => {
                if menu.selected > 0 {
                    menu.selected -= 1;
                }
            }
            KeyCode::Down => {
                if menu.selected < menu.items.len() - 1 {
                    menu.selected += 1;
                }
            }
            KeyCode::Enter => {
            
            }
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

pub fn run(terminal: &mut ratatui::DefaultTerminal, mut menu: Menu) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, &menu))?;
        if handle_events(&mut menu)? {
            break Ok(());
        }
    }
}

/*
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").white().block(Block::bordered().title("Hello middle"));
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
*/