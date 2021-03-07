extern crate dirs;
extern crate rustyline;

use std::collections::HashMap;
use std::env;
use std::fs;
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

fn parse_words(dictionary_contents: &str) -> Vec<&str> {
    dictionary_contents
        .lines()
        .filter(|word| !word.trim().is_empty())
        .collect()
}

#[cfg(test)]
#[test]
fn test_read_words_simple() {
    let test_input = "test\na\nstop";
    let expected_dict = vec!["test", "a", "stop"];
    let empty_vec: Vec<&str> = vec![];
    assert_eq!(parse_words(test_input), expected_dict);
    assert_eq!(parse_words(""), empty_vec);
    assert_eq!(parse_words("a"), vec!("a"));
    assert_eq!(parse_words("\n \n \n\n"), empty_vec)
}

/// Processes a dictionary file, converting the file to a HashMap of
/// signatures to lists of words.
fn process_dictionary(dict: &mut HashMap<String, Vec<String>>, words: Vec<&str>) {
    for word in words {
        dict.entry(word_signature(&word))
            .or_insert(Vec::new())
            .push(word.to_string());
    }
    dict.retain(|_, words| words.len() > 1);
}

fn process_command(cmd: &str, dictionary: &HashMap<String, Vec<String>>) -> Result<(), ()> {
    match cmd {
        "/help" => {
            println!("Enter a word and all anagrams will be printed.");
            println!("Commands:");
            println!("  WORD        Print anagrams for WORD.");
            println!("  /countsigs  Dump the count of signatures");
            println!("  /dump       Prints the entire dictionary (signatures and words).");
            println!("  /help       Print this help text.");
            println!("  /quit       Exit the program.");
        }
        "/dump" => {
            println!("Full dictionary:");
            println!("{:?}", dictionary);
        }
        "/countsigs" => {
            println!("Count: {}", dictionary.keys().len());
        }
        "/exit" | "/quit" => return Err(()),
        _ => {
            println!("Unsupported command: {}", cmd);
        }
    }
    Ok(())
}

fn determine_history_path() -> PathBuf {
    let mut history_path_buf = PathBuf::new();
    history_path_buf.push(dirs::data_dir().unwrap());
    history_path_buf.push("anagrams");
    history_path_buf.set_file_name("history.txt");
    history_path_buf
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
    let dictionary_file = match fs::read_to_string(dictionary_file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Unable to open dictionary file.");
            return;
        }
    };

    println!("Processing dictionary...");
    let mut dictionary = HashMap::new();
    let words = parse_words(&dictionary_file);
    process_dictionary(&mut dictionary, words);
    println!("");

    let history_path = determine_history_path();
    let history_path = history_path.as_path();

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_path);

    loop {
        match rl.readline("Word: ") {
            Ok(word) if word.trim().len() == 0 => continue,
            Ok(cmd) if cmd.starts_with("/") => {
                if let Err(_) = process_command(&cmd, &dictionary) {
                    break;
                }
            }
            Ok(word) => {
                if let Some(list) = dictionary.get(&word_signature(&word)) {
                    println!("Anagrams: {}", list.join(", "));
                } else {
                    eprintln!("No angrams found for '{}'.", word);
                }
                rl.add_history_entry(word);
            }
            Err(_) => {
                break;
            }
        }
        println!("");
    }
    let _ = rl.save_history(history_path);
}
