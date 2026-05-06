use std::{io::{self, Write}};
use std::process::Command;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub fn receive_input(input: &mut String) -> &mut String {
    io::stdout()
            .flush()
            .expect("Failed to flush stdout");

    input.clear();

    io::stdin()
        .read_line(input)
        .expect("Failed to read input.");

    return input;
}