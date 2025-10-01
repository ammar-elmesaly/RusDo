mod db;
mod ui;
mod menu;
mod task;
mod view_tasks;
mod about;
mod add_task;

use std::process;
use ratatui;
use menu::{Menu, run_loop};


fn main() {
    let mut terminal = ratatui::init();
    
    run_loop(&mut terminal).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    ratatui::restore();
}