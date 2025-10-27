use dal::json_file::get_all as get_all_handle;
use crate::structs::{ToDoItem, AllToDoItems};
use glue::errors::NanoServiceError;

pub async fn get_all() -> Result<AllToDoItems, NanoServiceError> {
    Ok(AllToDoItems::from_hashmap(get_all_handle::<ToDoItem>()?))
}