use crate::utils;
use crate::library_manager;

pub fn main_menu() {
    let mut input: String = String::new();
    let mut run: bool = true;

    while run {
        utils::clear_terminal();
        println!("|-------------------------------------------------------|");
        println!("|                     Main Menu                         |");
        println!("|-------------------------------------------------------|");
        println!("| [1]: View Library                                     |");
        println!("| [2]: Add Library Item                                 |");
        println!("| [q]: Exit Program                                     |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|-------------------------------------------------------|");
        print!("Enter Choice: ");
        
        utils::receive_input(&mut input);

        match input.trim() {
            "1" => {
                library_menu(&mut run, &mut input);
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

fn library_menu(run: &mut bool, input: &mut String) {
    while *run {
        utils::clear_terminal();
        println!("|-------------------------------------------------------|");
        println!("|                        Library                        |");
        println!("|-------------------------------------------------------|");
        println!("| [1]: Previous Page                                    |");
        println!("| [2]: Next Page                                        |");
        println!("| [q]: Exit Program                                     |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|                                                       |");
        println!("|-------------------------------------------------------|");
        print!("Enter Choice: ");
        
        utils::receive_input(input);

        match input.trim() {
            "q" => {
                *run = false;
            }
            _ => {
                println!("Invalid Choice. Press enter to try again.");
                utils::receive_input(input);
            }
        }
    }
}