use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::Connection;

use crate::{task::Task, ui::draw_view};

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            _ => {}
        }

        _ => {}
    }
    Ok(false)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal, conn: &Connection) -> std::io::Result<()>  {
    loop {
        let tasks = Task::all(conn).unwrap();
        terminal.draw(|frame| draw_view(frame, tasks))?;
        if handle_events()? {
            return Ok(());
        }
    }
}