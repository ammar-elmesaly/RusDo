use std::process;
use rusdo::{run, Menu};
use ratatui;

fn main() {
    let menu = Menu::init();

    let mut terminal = ratatui::init();
    
    run(&mut terminal, menu).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    ratatui::restore();
} 