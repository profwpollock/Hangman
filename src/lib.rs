#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]

//! # The library code for the hangman game

use std::path::PathBuf;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use rand::Rng;

const NUMBER_OF_GUESSES: i32 = 7;
const WORD_LIST_FILE: &str = "words.txt";

/// Play one complete game of hangman.  Return Err if user quits.
pub fn play_game() -> Result<(), String> {
    let word: String = get_word();
    let word_len = word.len();
    let mut letters_guessed: String = "_ ".repeat(word_len).trim().to_string();
    let mut num_guesses_left = NUMBER_OF_GUESSES;
    let mut previous_guesses = String::new();
    let mut guess: String;

    while num_guesses_left > 0 {
        ui(&letters_guessed, num_guesses_left, &previous_guesses);
        guess = get_guess();
        if guess.len() == 0 {
            continue;
        }
        guess = guess.to_lowercase();
        if guess.len() > 1 {
            if guess == "quit".to_string() {
                return Err("done".to_string());
            }
            // check for word match
            if guess == word {
                println!("Congratulations!  You won!!");
                return Ok(());
            }
        } else {  // guess was one letter
            // check for letter in the word:
            if word.contains(&guess) {
                // update letters_guessed
                ; // TODO
            } else {
                // update previous_guesses list
                previous_guesses.push_str(&guess);
                let mut letters: Vec<char> = previous_guesses.chars().collect();
                letters.sort_unstable();
                previous_guesses = letters.into_iter().collect();            
            }

        }
        num_guesses_left -= 1;
    }
    println!("No guesses left!  (The word was \"{}\".)", word);
    Ok(())
}

/// Program resources are in the crate_root/assets directory by default.
pub fn get_assets_directory() -> PathBuf {
    let mut assets_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_dir.push("assets");
    assets_dir
}

/// Return a random word from a file of words containing one word per line.
fn get_word() -> String {
    // locate and open the word list file:
    let mut path = get_assets_directory();
    path.push(WORD_LIST_FILE);
    let err_message =
        format!("Unable to open file \"{}\".", WORD_LIST_FILE);
    let word_list = BufReader::new(File::open(path.as_path())
        .expect(err_message.as_str()));

    // count words (lines) in the file:
    let mut num_words = 0;
    for _ in word_list.lines() {
        num_words += 1;
    }

    // Select a random word from file:
    let num = rand::thread_rng().gen_range(1..=num_words);
    let word_list = BufReader::new(File::open(path.as_path())
    .expect(err_message.as_str()));
    let mut word_iter = word_list.lines();
    let word: String = word_iter
        .nth(num - 1)
        .unwrap().unwrap();

    println!("----------\nToday's secret word is \"{}\".", word);  // scaffold
    word
}

/// Gets user input:
fn get_guess() -> String {
    let mut guess = String::new();
    print!("\nWhat is your guess (type \"quit\" to quit)? ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess);
    guess = guess.trim().to_string();
    guess
}
/// Display user interface: underscores for unguessed letters,
/// number of guesses left, previous guesses, and a prompt:
fn ui (letters_guessed: &str, num_guesses_left: i32, previous_guesses: &str) {
    print!("\n{}", letters_guessed);
    println!("    Guesses left: {}", num_guesses_left);
    if previous_guesses.len() != 0 {
        print!("    Previous Guesses: {}", previous_guesses);
    }
}
