use dal::json_file::get_all as get_all_handle;
use crate::structs::{ToDoItem, AllToDoItems};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

pub async fn get_all() -> Result<AllToDoItems, NanoServiceError> {
    Ok(AllToDoItems::from_hashmap(get_all_handle::<ToDoItem>()?))
}

pub async fn get_by_name(name: &str) -> Result<ToDoItem, NanoServiceError> {
    Ok(get_all_handle::<ToDoItem>()?
       .remove(name) // Removes a key from the map, returning the value at the key if the key was previously in the map.
       .ok_or(NanoServiceError::new(
        format!("Item with name {} not found", name),
        NanoServiceErrorStatus::NotFound,
    ))?)
}