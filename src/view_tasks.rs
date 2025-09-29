use std::error::Error;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::Connection;

use crate::{task::{Task, TaskList}, ui::draw_view};

fn handle_events(task_list: &mut TaskList, conn: &Connection) -> Result<bool, Box<dyn Error>> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up => task_list.move_prev(),
            KeyCode::Down => task_list.move_next(),
            KeyCode::Enter => task_list.check_current_task(conn)?,
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            _ => {}
        }

        _ => {}
    }
    Ok(false)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal, conn: &Connection) -> Result<(), Box<dyn Error>>  {
    let mut task_list = Task::all(conn).unwrap();
    loop {
        terminal.draw(|frame| draw_view(frame, &task_list))?;
        if handle_events(&mut task_list, conn)? {
            return Ok(());
        }
    }
}