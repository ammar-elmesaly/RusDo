use rusqlite::{Connection, Result};
use std::{env, fs, path::PathBuf};

pub fn init_db() -> Result<Connection> {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    
    path.push(".rusdo-data");
    fs::create_dir_all(&path).unwrap();

    path.push("tasks.db");  // push database file name to path
    let conn = Connection::open(&path)?;
    Ok(conn)
}
