use std::error::Error;

use crate::{task::{TaskList, Task}, ui::draw_add_task};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::Connection;
use tui_input::{Input, backend::crossterm::EventHandler};

pub enum SelectedInput {
    Title,
    Desc,
}

pub fn handle_events(
    title_input: &mut Input,
    desc_input: &mut Input,
    selected_input: &mut SelectedInput,
    conn: &Connection,
    task_list: &mut TaskList,
) -> Result<bool, Box<dyn Error>> {
    let event = event::read()?;
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc => return Ok(true),
            KeyCode::Up => {*selected_input = SelectedInput::Title}
            KeyCode::Down => {*selected_input = SelectedInput::Desc}
            KeyCode::Enter => { 
                Task::insert(conn, title_input.value(), desc_input.value(), task_list)?;
                return Ok(true);
            },
            _ => {
                if let SelectedInput::Title = selected_input {
                    title_input.handle_event(&event); 
                } else {
                    desc_input.handle_event(&event);
                }
            }
        },
        _ => {}
    }

    Ok(false)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal, conn: &Connection, task_list: &mut TaskList) -> Result<(), Box<dyn Error>> {
    let mut title_input = Input::new(String::new());
    let mut desc_input = Input::new(String::new());
    let mut selected_input = SelectedInput::Title;

    loop {
        terminal.draw(|frame| draw_add_task(frame, title_input.value(), desc_input.value(), &selected_input))?;
        if handle_events(&mut title_input, &mut desc_input, &mut selected_input, conn, task_list)? {
            return Ok(());
        }
    }
}