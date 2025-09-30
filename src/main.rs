mod db;
mod ui;
mod menu;
mod task;
mod view_tasks;
mod about;

use std::process;
use ratatui;
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
            KeyCode::Enter => return Ok(menu.current_action()),
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
    let mut show_message = false;

    loop {
        terminal.draw(|frame| draw_menu(frame, &menu, show_message))?;

        // For each action, we run a sub-run function, when that sub-run function returns, it returns here.
        match handle_events(&mut menu)? {
            MenuAction::Exit => break Ok(()),
            MenuAction::ViewTasks => show_message = view_tasks::run_loop(terminal, &conn)?,
            MenuAction::About => { about::run_loop(terminal)?; show_message = false },
            _ => { }
        };
    }
}

fn main() {
    let mut terminal = ratatui::init();
    
    run(&mut terminal).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    ratatui::restore();
}