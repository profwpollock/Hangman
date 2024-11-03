//! Play a game of hangman.  This version uses the console (TUI).
//! 

use hangman::*;
use std::env;
use regex::Regex;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

fn main() {
    // Get the name of the program, er, programatically:
    let cmd_line: Vec<String> = env::args().collect();
    let mut program_name = cmd_line[0].clone();
    // Strip the path and any trailing ".exe" from the executable name:
    let re = Regex::new(r".*[/\\](?P<name>[^.]+)(.exe)?").unwrap();
    program_name = re.replace(&program_name, "$name").to_string();
    PROGRAM_NAME.get_or_init(|| program_name);

    // Some scaffold code:
    println!("The path to the assets directory is {}", get_assets_directory().display());
    err("Program starts");
}

fn err (msg: &str) {
    eprintln!("Usage: {}", PROGRAM_NAME.get().unwrap());
    eprintln!("{}", msg);
    eprintln!("Program will now abort");
    std::process::exit(1);  // 0 = success, any other = failure
}

#[cfg(test)]
mod unit_tests;
