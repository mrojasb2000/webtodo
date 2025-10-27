use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};
use crate::enums::TaskStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
            TaskStatus::DONE => write!(f, "Done: {}", self.title),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllToDoItems {
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>) -> AllToDoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items.values() {
            match item.status {
                TaskStatus::PENDING => pending.push(item.clone()),
                TaskStatus::DONE => done.push(item.clone()),
            }
        }

        AllToDoItems { pending, done }
    }
}