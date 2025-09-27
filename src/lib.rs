mod ui;

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ui::draw_menu;

struct Item {
    content: &'static str,
    index: usize
}

impl Item {
    fn new(content: &'static str) -> Item {
        Item {
            content,
            index: 0
        }
    }
}
pub struct Menu {
    items: Vec<Item>,
    selected: usize
}

impl Menu {
    pub fn init() -> Menu {
        Menu {
            items: vec![Item::new("View Tasks"), Item::new("Add Task"), Item::new("Remove Task"), Item::new("Exit")],
            selected: 0
        }
    }
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
                if menu.selected == 3 {
                    return Ok(true);
                }
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
        terminal.draw(|frame| draw_menu(frame, &menu))?;
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