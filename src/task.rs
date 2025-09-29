use rusqlite::{Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub desc: Option<String>,
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

    pub fn all(conn: &Connection) -> Result<TaskList> {
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
        Ok(TaskList { tasks, selected: 0 })
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub selected: usize
}

impl TaskList {
    pub fn move_next(&mut self) {
        self.selected = (self.selected + 1) % self.tasks.len();
    }

    pub fn move_prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.tasks.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn check_current_task(&mut self, conn: &Connection) -> Result<()> {
        let current_task = &mut self.tasks[self.selected];
        current_task.completed = !current_task.completed;
        conn.execute("UPDATE task SET completed = ?1 WHERE id = ?2;", [current_task.completed as u64, current_task.id])?;
        Ok(())
    }
}