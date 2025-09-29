mod ui;
mod menu;
mod view_tasks;
mod task;
mod db;

use std::error::Error;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ui::draw_menu;
use menu::MenuAction;
use menu::Menu;

use crate::task::Task;

fn handle_events(menu: &mut Menu) -> std::io::Result<MenuAction> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up => {
                menu.move_prev();
            }
            KeyCode::Down => {
                menu.move_next();
            }
            KeyCode::Enter => {
                match menu.current_action() {
                    MenuAction::ViewTasks => return Ok(MenuAction::ViewTasks),
                    MenuAction::Exit => return Ok(MenuAction::Exit),
                    _ => {}
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => return Ok(MenuAction::Exit),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(MenuAction::None)
}

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let conn = db::init_db()?;

    if let Ok(false) = Task::table_exists(&conn) {
        Task::create_table(&conn)?;
    }

    let mut menu = Menu::init();

    loop {
        terminal.draw(|frame| draw_menu(frame, &menu))?;

        // For each action, we run a sub-run function, when that sub-run function returns, it returns here.
        match handle_events(&mut menu) {
            Ok(MenuAction::Exit) => { break Ok(()) }
            Ok(MenuAction::ViewTasks) => { view_tasks::run_loop(terminal, &conn)?; }
            _ => { }
        };
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