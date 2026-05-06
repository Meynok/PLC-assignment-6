use std::{io::{self, Write}, ptr::null};
use std::process::Command;

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn receive_input() -> String {
    let mut input = String::new();
    io::stdout()
            .flush()
            .expect("Failed to flush stdout");

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    return input;
}

pub fn test_print() {
    let mut input: String = String::new();
    let mut run: bool = true;
    while run {
        clear_terminal();
        println!("|-------------------------------------------------------|");
        println!("|                     Main Menu                         |");
        println!("|-------------------------------------------------------|");
        println!("| [1]: View Library                                     |");
        println!("| [2]: Add Library Item                                 |");
        println!("| [3]: Edit Library Item                                |");
        println!("| [q]: exit program                                     |");
        println!("|-------------------------------------------------------|");
        print!("Enter Choice: ");

        input = receive_input();

        if input.trim() == "q" {run = false}
    }
}