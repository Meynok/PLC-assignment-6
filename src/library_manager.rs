use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

pub static LIBRARY_NAME: &str = "CS 510 Library Catalog";

#[derive(Serialize, Deserialize)]
pub enum ItemStatus {
    Available,
    CheckedOut,
}

#[derive(Serialize, Deserialize)]
pub struct ItemAuthor {
    pub first_name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize)]
pub struct LibraryItem {
    pub library_item_name: String,
    pub library_item_isbn: String,
    pub library_item_author: ItemAuthor,
    pub library_item_published_date: DateTime<Utc>,
    pub library_item_status: ItemStatus,
}

pub struct Library {
    pub items: Vec<LibraryItem>, 
}

impl Library {
    pub fn new() -> Self {
        Library { items: Vec::new() }
    }

    pub fn load_from_file(file_path: &str) -> Self {
        if let Ok(data) = fs::read_to_string(file_path) {
            if let Ok(items) = serde_json::from_str(&data) {
                return Library { items };
            }
        }

        Library::new()
    }

    pub fn save_to_file(&self, file_path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(&self.items) {
            let _ = fs::write(file_path, json);
        }
    }

    pub fn add_item(&mut self, item: LibraryItem) {
        self.items.push(item);
    }
}