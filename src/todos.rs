pub mod todo;

pub struct Todos {
    pub todos: Vec<todo::Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }

    pub fn add(&mut self, todo: todo::Todo) {
        self.todos.push(todo);
    }

    pub fn remove(&mut self, id: i32) {
        let todo_index = self.todos.iter().position(|x| x.id == id);

        if todo_index.is_some() {
            self.todos.remove(todo_index.unwrap());
        }
    }
}
