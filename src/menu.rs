use crate::utils;
use crate::library_manager::{Library, LibraryItem, ItemAuthor, ItemStatus};
use chrono::Utc;

const JSON_FILE_PATH: &str = "library_data.json";

pub fn main_menu() {
    let mut input: String = String::new();
    let mut run: bool = true;
    
    let mut library = Library::load_from_file(JSON_FILE_PATH);

    while run {
        utils::clear_terminal();
        println!("|-------------------------------------------------------|");
        println!("|                     Main Menu                         |");
        println!("|-------------------------------------------------------|");
        println!("| [1]: View Library                                     |");
        println!("| [2]: Add Library Item                                 |");
        println!("| [q]: Exit Program                                     |");
        for _ in 0..13 {
            println!("|                                                       |");
        }
        println!("|-------------------------------------------------------|");
        print!("Enter Choice: ");
        
        utils::receive_input(&mut input);

        match input.trim() {
            "1" => {
                library_menu(&mut run, &mut input, &mut library);
                library.save_to_file(JSON_FILE_PATH);
            }
            "2" => {
                add_item_menu(&mut input, &mut library);
                library.save_to_file(JSON_FILE_PATH);
            }
            "q" => {
                run = false;
            }
            _ => {
                println!("Invalid Choice. Press enter to try again.");
                utils::receive_input(&mut input);
            }
        }
    }
}

fn library_menu(run: &mut bool, input: &mut String, library: &mut Library) {
    let mut in_menu = true;
    let mut page: usize = 0;
    let items_per_page = 5;

    while in_menu && *run {
        utils::clear_terminal();
        let total_items = library.items.len();
        let total_pages = if total_items == 0 { 1 } else { (total_items + items_per_page - 1) / items_per_page };
        
        if page >= total_pages {
            page = total_pages.saturating_sub(1);
        }

        let start_index = page * items_per_page;
        let end_index = (start_index + items_per_page).min(total_items);

        println!("|-------------------------------------------------------|");
        println!("| {:^53} |", "Library");
        println!("|-------------------------------------------------------|");
        
        let mut lines_used = 0;
        for i in start_index..end_index {
            let item = &library.items[i];
            let title = format!("[{}]: {}", i + 1 - start_index, item.library_item_name);
            let display_string = title.chars().take(53).collect::<String>();
            println!("| {:<53} |", display_string);
            lines_used += 1;
        }

        for _ in lines_used..items_per_page {
            println!("|                                                       |");
        }

        println!("|-------------------------------------------------------|");
        println!("| Page {} of {:<42}  |", page + 1, total_pages);
        println!("| [n] Next Page  [p] Prev Page                          |");
        println!("| [b] Go Back    [q] Exit Program                       |");
        for _ in 0..7 {
            println!("|                                                       |");
        }
        println!("|-------------------------------------------------------|");
        print!("Enter Choice (or item #): ");
        
        utils::receive_input(input);

        match input.trim() {
            "n" => if page + 1 < total_pages { page += 1; },
            "p" => if page > 0 { page -= 1; },
            "b" => in_menu = false,
            "q" => *run = false,
            val => {
                if let Ok(num) = val.parse::<usize>() {
                    if num > 0 && num <= (end_index - start_index) {
                        let actual_idx = start_index + num - 1;
                        item_details_menu(run, input, library, actual_idx);
                    } else {
                        println!("Invalid Selection. Press enter to try again.");
                        utils::receive_input(input);
                    }
                } else {
                    println!("Invalid Choice. Press enter to try again.");
                    utils::receive_input(input);
                }
            }
        }
    }
}

fn item_details_menu(run: &mut bool, input: &mut String, library: &mut Library, item_index: usize) {
    let mut in_menu = true;
    while in_menu && *run {
        utils::clear_terminal();
        let item = &library.items[item_index];
        let status_str = match item.library_item_status {
            ItemStatus::Available => "Available",
            ItemStatus::CheckedOut => "Checked Out",
        };

        println!("|-------------------------------------------------------|");
        println!("| {:^53} |", "Item Details");
        println!("|-------------------------------------------------------|");
        println!("| Name: {:<47} |", item.library_item_name.chars().take(47).collect::<String>());
        
        let author_str = format!("{} {}", item.library_item_author.first_name, item.library_item_author.last_name);
        println!("| Author: {:<45} |", author_str.chars().take(45).collect::<String>());
        
        println!("| ISBN: {:<47} |", item.library_item_isbn.chars().take(47).collect::<String>());
        println!("| Status: {:<45} |", status_str);
        println!("|                                                       |");
        println!("|-------------------------------------------------------|");
        println!("| [1]: Edit Item Status                                 |");
        println!("| [b]: Go Back                                          |");
        println!("| [q]: Exit Program                                     |");
        for _ in 0..7 {
            println!("|                                                       |");
        }
        println!("|-------------------------------------------------------|");
        print!("Enter Choice: ");

        utils::receive_input(input);
        match input.trim() {
            "1" => {
                println!("Enter new status - [1] Available, [2] Checked Out:");
                utils::receive_input(input);
                match input.trim() {
                    "1" => library.items[item_index].library_item_status = ItemStatus::Available,
                    "2" => library.items[item_index].library_item_status = ItemStatus::CheckedOut,
                    _ => {
                        println!("Invalid status choice. Press enter to return.");
                        utils::receive_input(input);
                    }
                }
            },
            "b" => in_menu = false,
            "q" => *run = false,
            _ => {
                println!("Invalid Choice. Press enter to try again.");
                utils::receive_input(input);
            }
        }
    }
}

fn add_item_menu(input: &mut String, library: &mut Library) {
    utils::clear_terminal();
    
    println!("Enter Item Name:");
    utils::receive_input(input);
    let name = input.trim().to_string();

    if name.is_empty() {
        println!("Invalid name. Press enter to return to menu.");
        utils::receive_input(input);
        return;
    }

    println!("Enter Author First Name:");
    utils::receive_input(input);
    let first_name = input.trim().to_string();

    println!("Enter Author Last Name:");
    utils::receive_input(input);
    let last_name = input.trim().to_string();

    println!("Enter ISBN:");
    utils::receive_input(input);
    let isbn = input.trim().to_string();

    let new_item = LibraryItem {
        library_item_name: name,
        library_item_isbn: isbn,
        library_item_author: ItemAuthor { first_name, last_name },
        library_item_published_date: Utc::now(),
        library_item_status: ItemStatus::Available,
    };

    library.add_item(new_item);
    
    println!("Item successfully added! Press enter to continue.");
    utils::receive_input(input);
}