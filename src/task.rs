use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub desc: String,
    pub completed: bool
}

impl Task {

    pub fn table_exists(conn: &Connection) -> Result<bool> {
        conn.table_exists(None, "task")
    }

    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE task (
                id   INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                desc TEXT,
                completed INTEGER DEFAULT 0 NOT NULL
            )",
            (), // empty list of parameters.
        )?;

        Ok(())
    }

    pub fn insert(conn: &Connection, task: &Self) -> Result<()> {
        conn.execute(
            "INSERT INTO task (title, desc, completed) VALUES (?1, ?2, ?3)",
            (&task.title, &task.desc, &task.completed),
        )?;

        Ok(())
    }

    pub fn all(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT id, title, desc, completed FROM task")?;
        let iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                desc: row.get(2)?,
                completed: row.get(3)?,
            })
        })?;

        let mut tasks = Vec::new();
        for t in iter {
            tasks.push(t?);
        }
        Ok(tasks)
    }
}