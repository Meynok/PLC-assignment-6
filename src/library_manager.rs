use chrono::{DateTime, Utc};

struct ItemAuthor {
    first_name: String,
    last_name: String
}

pub struct LibraryItem {
    library_item_name: String,
    library_item_isbn: String,
    library_item_author: ItemAuthor,
    library_item_published_date: DateTime<Utc>,
}

fn get_library_item(library_item: &LibraryItem) -> &LibraryItem {
    return library_item;
}