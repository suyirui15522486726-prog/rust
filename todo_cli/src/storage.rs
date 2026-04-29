use crate::todo::Todo;

pub struct TodoStorage {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoStorage {
    pub fn new() -> Self {
        TodoStorage {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, content: String) -> u32 {
        let id = self.next_id;
        self.todos.push(Todo::new(id, content));
        self.next_id += 1;
        id
    }

    pub fn list(&self) {
        println!("ID  状态   内容");
        for todo in &self.todos {
            todo.display();
        }
    }

    pub fn done(&mut self, id: u32) -> bool {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.mark_done();
                true
            }
            None => false,
        }
    }

    pub fn remove(&mut self, id: u32) -> bool {
        let index = self.todos.iter().position(|t| t.id == id);
        match index {
            Some(i) => {
                self.todos.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn show(&self, id: u32) -> bool {
        match self.todos.iter().find(|t| t.id == id) {
            Some(todo) => {
                todo.display_detail();
                true
            }
            None => false,
        }
    }

    pub fn edit(&mut self, id: u32, content: String) -> bool {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.content = content;
                true
            }
            None => false,
        }
    }
}