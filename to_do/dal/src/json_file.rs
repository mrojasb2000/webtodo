use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};

/// Gets a file handle for JSON storage.
///
/// Reads the file path from the `JSON_STORE_PATH` environment variable.
/// If not defined, uses "tasks.json" as the default value.
/// The file is opened in read/write mode and created if it doesn't exist.
///
/// # Returns
///
/// * `Ok(File)` - Handle to the opened file
/// * `Err(String)` - Error message if file opening fails
fn get_handle() -> Result<File, String> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or_else(|_| "tasks.json".to_string());
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    Ok(file)
}

/// Retrieves all items stored in the JSON file.
///
/// Reads the JSON file content and deserializes it into a HashMap
/// where the key is a String (usually an ID), and the value is of generic type T.
///
/// # Type Parameters
///
/// * `T` - Type of items to deserialize. Must implement `DeserializeOwned`
///
/// # Returns
///
/// * `Ok(HashMap<String, T>)` - Map with all stored items
/// * `Err(String)` - Error message if reading or JSON parsing fails
///
/// # Examples
///
/// ```
/// let tasks: HashMap<String, Task> = get_all().unwrap();
/// ```
pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, String>{
    let mut file = get_handle()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Error reading file: {}", e))?;
    let tasks: HashMap<String, T> = serde_json::from_str(&contents)
        .map_err(|e| format!("Error parsing JSON: {}", e))?;
    Ok(tasks)
}

/// Saves all items to the JSON file.
///
/// Serializes the complete HashMap to JSON with readable format (pretty-print)
/// and overwrites the file content.
///
/// # Type Parameters
///
/// * `T` - Type of items to serialize. Must implement `Serialize`
///
/// # Arguments
///
/// * `tasks` - Reference to the HashMap with all items to save
///
/// # Returns
///
/// * `Ok(())` - If the operation was successful
/// * `Err(String)` - Error message if serialization or writing fails
///
/// # Examples
///
/// ```
/// let mut tasks = HashMap::new();
/// tasks.insert("1".to_string(), my_task);
/// save_all(&tasks).unwrap();
/// ```
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), String>{
    let mut file = get_handle()?;
    let json = serde_json::to_string_pretty(tasks).map_err(|e| format!("Error serializing JSON: {}", e))?;
    file.write_all(json.as_bytes()).map_err(|e| format!("Error writing to file: {}", e))?;
    Ok(())
}

/// Retrieves a single item from JSON storage by its ID.
///
/// Searches for a specific item in the JSON file using its identifier.
/// First retrieves all items and then searches for the one matching the provided ID.
///
/// # Type Parameters
///
/// * `T` - Type of item to retrieve. Must implement `DeserializeOwned` and `Clone`
///
/// # Arguments
///
/// * `id` - Unique identifier of the item to search for
///
/// # Returns
///
/// * `Ok(T)` - The found item
/// * `Err(String)` - Error message if the item is not found or reading fails
///
/// # Examples
///
/// ```
/// let task: Task = get_one("123").unwrap();
/// ```
pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, String>{
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(format!("Task with id {} not found", id))
    }
}

/// Saves a single item to JSON storage.
///
/// Updates or inserts an item in the JSON file. If the ID already exists,
/// the item is updated; if it doesn't exist, a new one is created.
/// If the file doesn't exist or is empty, a new HashMap is created.
///
/// # Type Parameters
///
/// * `T` - Type of item to save. Must implement `Serialize`, `DeserializeOwned` and `Clone`
///
/// # Arguments
///
/// * `id` - Unique identifier of the item
/// * `task` - Reference to the item to save
///
/// # Returns
///
/// * `Ok(())` - If the operation was successful
/// * `Err(String)` - Error message if the operation fails
///
/// # Examples
///
/// ```
/// let task = Task::new("My task");
/// save_one("123", &task).unwrap();
/// ```
pub fn save_one<T>(id: &str, task: &T) -> Result<(), String> where T: Serialize + DeserializeOwned + Clone {
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}


/// Deletes an item from the JSON storage by its ID.
///
/// Searches for and removes the item corresponding to the provided identifier.
/// If the item does not exist, no further action is taken.
///
/// # Type Parameters
///
/// * `T` - Type of the stored items. Must implement `Serialize`, `DeserializeOwned`, and `Clone`
///
/// # Arguments
///
/// * `id` - Unique identifier of the item to delete
///
/// # Returns
///
/// * `Ok(())` - If the operation was successful
/// * `Err(String)` - Error message if the operation fails
///
/// # Example
///
/// ```
/// delete_one::<Task>("123").unwrap();
/// ```
pub fn delete_one<T>(id: &str) -> Result<(), String> where T: Serialize + DeserializeOwned + Clone {
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.remove(id);
    save_all(&tasks)
}