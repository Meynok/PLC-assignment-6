# CS 510 Assignment 6

**Author:** Brandon Nguyen
**Course:** CS 510 Programming Language Concepts, Spring 2026  

## Overview
This is a terminal-based Library Manager application written in Rust. It kinda sucks but ¯\_(ツ)_/¯

---

## Prerequisites
To compile and run this program, you must have the Rust toolchain (`rustc` and `cargo`) installed. 

* **Unix/macOS/Linux:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* **Windows:** Download `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

Verify your installation by running:
`cargo --version`

## How to Compile and Run

1. Navigate to the root of the project (the folder containing the `Cargo.toml` file).
   `cd path/to/PLC-assignment-6`
2. To run, execute the following command. Cargo will automatically fetch the required crates (`serde`, `serde_json`, `chrono`), compile the program, and execute the binary.
   `cargo run`
