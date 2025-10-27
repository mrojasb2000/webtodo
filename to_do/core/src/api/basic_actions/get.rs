use dal::json_file::get_all as get_all_handle;
use crate::structs::{ToDoItem, AllToDoItems};

pub async fn get_all() -> Result<AllToDoItems, String> {
    Ok(AllToDoItems::from_hashmap(get_all_handle::<ToDoItem>()?))
}