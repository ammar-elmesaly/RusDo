#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    ViewTasks = 0,
    AddTask = 1,
    RemoveTask = 2,
    Exit = 3,
}

impl TryFrom<usize> for MenuAction {

    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MenuAction::ViewTasks),
            1 => Ok(MenuAction::AddTask),
            2 => Ok(MenuAction::RemoveTask),
            3 => Ok(MenuAction::Exit),
            _ => Err(())
        }
    }
}

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
                MenuItem::new("Remove Task", MenuAction::RemoveTask),
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

    pub fn current_action(&self) -> MenuAction {
        MenuAction::try_from(self.selected).unwrap()
    }
}
