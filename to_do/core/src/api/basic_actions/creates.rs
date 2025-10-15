use std::fmt;
use dal::json_file::save_one;

use crate::structs::{
    done::Done,
    pending::Pending,
};

use crate::enums::TaskStatus;

pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypes::Done(done) => write!(f, "{}", done.super_struct.title),
            ItemTypes::Pending(pending) => write!(f, "{}", pending.super_struct.title),
        }
    }
}

pub fn create(title: &str, status: TaskStatus) -> Result<ItemTypes, String> {
    let _ = save_one(&title.to_string(), &status)?;
    match status {
        TaskStatus::PENDING => {
            Ok(ItemTypes::Pending(Pending::new(title)))
        },
        TaskStatus::DONE => {
            Ok(ItemTypes::Done(Done::new(title)))
        },
    }
}