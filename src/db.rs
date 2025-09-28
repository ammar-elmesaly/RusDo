use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("./data/tasks.db")?;
    Ok(conn)
}
