#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String) -> Self {
        Self {
            id: id,
            title: title,
            completed: false,
        }
    }
}
