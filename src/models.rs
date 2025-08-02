use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TodoItem {
    id: u128,
    description: String,
    priority: Option<u8>,
    due_date: Option<NaiveDate>,
    completed: bool,
}

impl TodoItem {
    pub fn new(description: String, priority: Option<u8>, due_date: Option<NaiveDate>) -> Self {
        // let i = Uuid::new_v4().as_u128();
        Self {
            id: Uuid::new_v4().as_u128(),
            description,
            priority,
            due_date,
            completed: false,
        }
    }
    pub fn set_completed(&mut self, b: bool) {
        self.completed = b;
    }
    pub fn get_id(& self) -> u128 {
        self.id
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