use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::ui::draw_view;

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        }

        _ => {}
    }
    Ok(false)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()>  {
    loop {
        terminal.draw(|frame| draw_view(frame))?;
        if handle_events()? {
            return Ok(());
        }
    }
}