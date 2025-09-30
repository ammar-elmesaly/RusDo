use std::error::Error;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::ui::draw_about;

pub fn handle_events() -> Result<bool, Box<dyn Error>>{
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            _ => {}
        }
        _ => {}
    }

    Ok(false)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>>{
    loop {
        terminal.draw(|frame| draw_about(frame))?;
        if handle_events()? {
            return Ok(());
        }
    }
}
