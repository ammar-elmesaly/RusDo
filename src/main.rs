mod db;
mod ui;
mod menu;
mod task;
mod view_tasks;
mod about;
mod add_task;

use std::process;
use ratatui;


fn main() {
    let mut terminal = ratatui::init();
    
    menu::run_loop(&mut terminal).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    ratatui::restore();
}