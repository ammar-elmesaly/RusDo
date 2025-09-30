use std::error::Error;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::Connection;

use crate::{task::Task, ui::{draw_view, draw_view_task}};

pub enum Action {
    Exit,
    Stay,
    ViewTask,
    MovePrev,
    MoveNext,
    CheckTask,
    UncheckTask,
    DeleteTask
}

pub fn handle_events() -> Result<Action, Box<dyn Error>> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up => return Ok(Action::MovePrev),
            KeyCode::Down => return Ok(Action::MoveNext),
            KeyCode::Enter => return Ok(Action::ViewTask),
            KeyCode::Char('d') => return Ok(Action::DeleteTask),
            KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Exit),
            _ => {}
        }

        _ => {}
    }
    Ok(Action::Stay)
}

fn handle_view_task_events(confirm_selection: &mut usize) -> Result<Action, Box<dyn Error>> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Exit),
            KeyCode::Right => { if *confirm_selection == 0 {*confirm_selection = 1} }
            KeyCode::Left => { if *confirm_selection == 1 {*confirm_selection = 0} }
            KeyCode::Enter => {
                if *confirm_selection == 0 {
                    return Ok(Action::CheckTask);

                } else {
                    return Ok(Action::UncheckTask);
                }
            }
            _ => {}
        }

        _ => {}
    }
    Ok(Action::Stay)
}

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal, conn: &Connection) -> Result<(), Box<dyn Error>>  {
    let mut task_list = Task::all(conn)?;
    loop {
        terminal.draw(|frame| draw_view(frame, &task_list))?;
        match handle_events()? {
            Action::MoveNext => task_list.move_next(),
            Action::MovePrev => task_list.move_prev(),
            Action::Exit => return Ok(()),
            Action::ViewTask => {
                let task = task_list.current_task();
                match view_task_loop(terminal, task)? {
                    Action::CheckTask => task_list.check_current_task(conn)?,
                    Action::UncheckTask => task_list.uncheck_current_task(conn)?,
                    _ => {}
                }
            }
            Action::DeleteTask => {
                
            }
            _ => {}
        }
    }
}

pub fn view_task_loop(terminal: &mut ratatui::DefaultTerminal, task: &Task) -> Result<Action, Box<dyn Error>>  {
    let mut confirm_selection = 0;
    loop {
        terminal.draw(|frame| draw_view_task(frame, &task, confirm_selection))?;
        let result = handle_view_task_events(&mut confirm_selection);
        let Ok(Action::Stay) = result else {
            return result;
        }; 
    }
}