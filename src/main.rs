#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]

//! Play a game of hangman.  This version uses the console (TUI).
//!

use hangman::*;
use regex::Regex;
use std::env;

/// Initialize everything, then repeatedly play a game until user quits.
pub fn main() {
    // Get the name of the program, er, programatically:
    let cmd_line: Vec<String> = env::args().collect();
    let mut program_name = cmd_line[0].clone();
    // Strip the path and any trailing ".exe" from the executable name:
    let re = Regex::new(r".*[/\\](?P<name>[^.]+)(.exe)?").unwrap();
    program_name = re.replace(&program_name, "$name").to_string();
    PROGRAM_NAME.get_or_init(|| program_name);

    println!("\nHANGMAN\n");
    while play_game().is_ok() {}
    println!("Thanks for playing!");
}

#[cfg(test)]
mod unit_tests;
