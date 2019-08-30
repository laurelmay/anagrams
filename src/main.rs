use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::iter::FromIterator;

/// Common way to get the signature of a particular word.
fn word_signature(word: &str) -> String {
    let lowercase = word.to_lowercase();
    let mut chars: Vec<char> = lowercase.chars().collect();
    // sort the letters within the word, allowing all words containing the
    // same letters to have the same signature
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

/// Processes a dictionary file, converting the file to a HashMap of
/// signatures to lists of words.
fn process_dictionary(dict: &mut HashMap<String, Vec<String>>, file: &mut File) {
    for line in BufReader::new(file).lines() {
        if let Ok(word) = line {
            dict.entry(word_signature(&word)).or_insert(Vec::new()).push(word);
        } else {
            eprintln!("Unable to process {:?}", line);
        }
    }
}

/// Prints program usage information.
fn usage(args: Vec<String>) {
    println!("Usage: {} FILE", args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(args);
        return;
    }

    let dictionary_file = &args[1];

    let mut file = match File::open(dictionary_file) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Unable to open dictionary file.");
            return;
        }
    };

    println!("Processing dictionary...");
    let mut dict = HashMap::new();
    process_dictionary(&mut dict, &mut file);
    println!("");

    let stdin = io::stdin();
    loop {
        print!("Word: ");
        // Ensure the prompt gets printed
        if let Err(_) = io::stdout().flush() {
            // Exit if unable to flush stdout
            return;
        }

        let mut word = String::new();
        match stdin.read_line(&mut word) {
            // Handle when EOF is given
            Ok(len) if len == 0 => {
                // Finish the line the prompt was given on. Together, these
                // should result in similar behavior between ^C and ^D being
                // entered.
                println!("");
                break;
            },
            Ok(_) => { },
            Err(_) => {
                eprintln!("Unable to read given input.");
            }
        };

        // Remove the \n from the end of the string
        word.pop();

        if word.len() == 0 {
            continue;
        }

        if let Some(list) = dict.get(&word_signature(&word)) {
            println!("Anagrams: {}", list.join(", "));
        } else {
            println!("Error: '{}' not in dictionary.", word);
        }

        println!("");
    }
}
