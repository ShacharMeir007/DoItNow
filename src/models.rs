use chrono::NaiveDate;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    description: String,
    priority: Option<u8>,
    due_date: Option<NaiveDate>,
    completed: bool,
}

impl TodoItem {
    pub fn new(description: String, priority: Option<u8>, due_date: Option<NaiveDate>) -> Self {
        Self {
            description,
            priority,
            due_date,
            completed: false,
        }
    }
    pub fn set_completed(&mut self, b: bool) {
        self.completed = b;
    }
}

// struct TodoList {
//     id: u32,
//     name: String,
//     items: Vec<TodoItem>
// }

// impl TodoList {
//     fn add_item(&mut self, item: TodoItem) {
//         self.items.push(item);
//     }
// }