extern crate dirs;
extern crate rustyline;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::PathBuf;

use rustyline::Editor;

/// Common way to get the signature of a particular word.
fn word_signature(word: &str) -> String {
    let lowercase = word.to_lowercase();
    let mut chars: Vec<char> = lowercase.chars().collect();
    // sort the letters within the word, allowing all words containing the
    // same letters to have the same signature
    chars.sort_by(|a, b| a.cmp(b));
    let chars = chars;
    String::from_iter(chars)
}

#[cfg(test)]
#[test]
fn word_sig_test() {
    assert_eq!(word_signature("test"), *"estt");
    assert_eq!(word_signature(""), *"");
    assert_eq!(word_signature("a"), *"a");
    assert_eq!(word_signature("431"), *"134");
    assert_eq!(word_signature("stop"), *"opst");
}

/// Processes a dictionary file, converting the file to a HashMap of
/// signatures to lists of words.
fn process_dictionary(dict: &mut HashMap<String, Vec<String>>, words: Vec<String>) {
    for word in words {
        dict.entry(word_signature(&word)).or_insert(Vec::new()).push(word.to_string());
    }
}

/// Prints program usage information.
fn usage(program_name: &str) {
    println!("Usage: {} FILE", program_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
        return;
    }

    let dictionary_file_path = &args[1];
    let dictionary_file = match File::open(dictionary_file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Unable to open dictionary file.");
            return;
        }
    };

    println!("Processing dictionary...");
    let mut dictionary = HashMap::new();
    let dict_lines = BufReader::new(dictionary_file).lines();
    let mut words = vec!();
    for line in dict_lines {
        if let Ok(word) = line {
            words.push(word);
        } else {
            eprintln!("Unable to process line: {:?}", line);
        }
    }
    process_dictionary(&mut dictionary, words);
    println!("");

    let mut history_path_buf = PathBuf::new();
    history_path_buf.push(dirs::data_dir().unwrap());
    history_path_buf.push("anagrams");
    history_path_buf.set_file_name("history.txt");
    let history_path = history_path_buf.as_path();

    let mut rl = Editor::<()>::new();
    let _ =  rl.load_history(history_path);

    loop {
        match rl.readline("Word: ") {
            Ok(word) => {
                if word.len() == 0 {
                    continue;
                } else if word == "/dump" {
                    println!("Full dictionary:");
                    println!("{:?}", dictionary);
                } else if word == "/countsigs" {
                    println!("Count: {}", dictionary.keys().len());
                } else if word == "/quit" {
                    break;
                } else if word == "/help" {
                    println!("Enter a word and all anagrams will be printed.");
                    println!("Commands:");
                    println!("  WORD        Print anagrams for WORD.");
                    println!("  /countsigs  Dump the count of signatures");
                    println!("  /dump       Prints the entire dictionary (signatures and words).");
                    println!("  /help       Print this help text.");
                    println!("  /quit       Exit the program.");
                } else if word.starts_with("/") {
                    eprintln!("Unsupported command: '{}'", word);
                } else if let Some(list) = dictionary.get(&word_signature(&word)) {
                    println!("Anagrams: {}", list.join(", "));
                    rl.add_history_entry(word);
                } else {
                    eprintln!("Error: '{}' not in dictionary.", word);
                    rl.add_history_entry(word);
                }
            },
            Err(_) => {
                break;
            }
        }
        println!("");
    }
    let _ =  rl.save_history(history_path);
}

