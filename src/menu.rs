pub mod view_tasks;
pub mod about;
pub mod add_task;

use std::error::Error;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::ui::draw_menu;

use crate::task::Task;

#[repr(usize)]
pub enum MenuAction {  
    ViewTasks = 0,
    AddTask = 1,
    About = 2,
    Exit = 3,
    None = 4,
}

impl TryFrom<usize> for MenuAction {  // Map Menu selected (usize) to a Menu::Action

    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MenuAction::ViewTasks),
            1 => Ok(MenuAction::AddTask),
            2 => Ok(MenuAction::About),
            3 => Ok(MenuAction::Exit),
            _ => Err(())
        }
    }
}

#[allow(dead_code)]
pub struct MenuItem {
    pub content: &'static str,
    pub action: MenuAction,
}

impl MenuItem {
    pub fn new(content: &'static str, action: MenuAction) -> MenuItem {
        MenuItem { content, action }
    }
}
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub selected: usize,
}

impl Menu {
    pub fn init() -> Self {
        Menu {
            items: vec![
                MenuItem::new("View Tasks", MenuAction::ViewTasks),
                MenuItem::new("Add Task", MenuAction::AddTask),
                MenuItem::new("About", MenuAction::About),
                MenuItem::new("Exit", MenuAction::Exit),
            ],
            selected: 0,
        }
    }

    pub fn move_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn move_prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn current_action(&self) -> MenuAction {  // Current selected action (view tasks, add, .. etc)
        MenuAction::try_from(self.selected).unwrap()
    }
}

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

pub fn run_loop(terminal: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let conn = crate::db::init_db()?;

    if let Ok(false) = Task::table_exists(&conn) {
        Task::create_table(&conn)?;
    }

    let mut menu = Menu::init();
    let mut show_message = false;

    let mut task_list = Task::all(&conn, 0)?;

    loop {
        terminal.draw(|frame| draw_menu(frame, &menu, show_message))?;

        // For each action, we run a sub-run function, when that sub-run function returns, it returns here.
        match handle_events(&mut menu)? {
            MenuAction::Exit => break Ok(()),
            MenuAction::ViewTasks => show_message = view_tasks::run_loop(terminal, &conn, &mut task_list)?,
            MenuAction::AddTask => { add_task::run_loop(terminal, &conn, &mut task_list)?; show_message = false},
            MenuAction::About => { about::run_loop(terminal)?; show_message = false },
            _ => { }
        };
    }
}