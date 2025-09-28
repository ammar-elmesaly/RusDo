use std::process;
use ratatui;
use rusqlite::{params, Connection, Result};
use rusdo::run;

fn main() {
    let mut terminal = ratatui::init();
    
    run(&mut terminal).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    ratatui::restore();
}