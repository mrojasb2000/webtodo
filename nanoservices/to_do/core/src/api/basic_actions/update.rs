use dal::json_file::{get_all as get_all_handle, save_all};
use crate::structs::ToDoItem;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

/// Updates an existing to-do item.
pub fn update(item: ToDoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all_handle::<ToDoItem>()?;
    if !all_items.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with title {} not found", item.title),
            NanoServiceErrorStatus::NotFound
        ));
    }
    all_items.insert(item.title.clone(), item.clone());
    save_all(&all_items)?;
    Ok(())
}
