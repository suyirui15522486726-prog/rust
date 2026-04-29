pub struct Todo {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, content: String) -> Self {
        Todo {
            id,
            content,
            completed: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) {
        let status = if self.completed { "[x]" } else { "[ ]" };
        println!("{} {} {}", self.id, status, self.content);
    }

    pub fn display_detail(&self) {
        let status = if self.completed { "Done" } else { "Not Done" };
        println!("ID: {}", self.id);
        println!("Status: {}", status);
        println!("Content: {}", self.content);
    }
}