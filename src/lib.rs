mod ui;
mod menu;

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ui::draw_menu;
use menu::MenuAction;

pub use menu::Menu;

use crate::ui::draw_view;


fn handle_events(menu: &mut Menu) -> std::io::Result<&str> {
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
                    MenuAction::ViewTasks => { return Ok("view");}
                    MenuAction::Exit => { return Ok("exit") }
                    _ => {}
                }
            }
            KeyCode::Char('q') => return Ok("exit"),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok("")
}

pub fn run(terminal: &mut ratatui::DefaultTerminal, mut menu: Menu) -> std::io::Result<()> {
    loop {
        match handle_events(&mut menu) {
            Ok("exit") => { break Ok(()) }
            Ok("view") => { terminal.draw(|frame| draw_view(frame, &menu))?;}
            _ => { terminal.draw(|frame| draw_menu(frame, &menu))?; }
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