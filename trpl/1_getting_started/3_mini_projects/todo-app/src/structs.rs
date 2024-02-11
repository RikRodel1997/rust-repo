pub struct ToDo {
    pub id: String,
    pub message: String,
    pub active: bool,
}

pub struct ToDoList {
    pub id: String,
    pub todos: Vec<ToDo>,
}
