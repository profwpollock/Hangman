use hangman::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Integration Tests go here
#[test]
fn integration_test() {
    assert_eq!(4, 2 + 2);
}

#[test]
fn dictionary_test() {
    // locate and open the word list file:
    let mut path = get_assets_directory();
    path.push(WORD_LIST_FILE);
    let err_message = format!("Unable to open file \"{}\".", WORD_LIST_FILE);
    let word_list = BufReader::new(
        File::open(path.as_path())
            .unwrap_or_else(|_| panic!("{}", err_message)),
    );
    
    let mut num_words = 0;
    for word in word_list.lines() {
        let word = word.unwrap();
        num_words += 1;
        assert!( word.is_ascii(),
            "non-ascii characters found in {word} at {num_words}");
    
        let is_lowercase = word.chars()
            .all(|c| c.is_ascii_lowercase() || c == '-');
        assert!(is_lowercase,
            "non-lowercase characters found in {word} at {num_words}");
        assert!(num_words > 0, "Word list is empty!");
    }
}
