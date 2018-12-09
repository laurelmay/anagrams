use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::iter::FromIterator;

/// Common way to get the signature of a particular word.
fn word_signature(word: String) -> String {
    let lowercase = word.to_lowercase();
    let mut chars: Vec<char> = lowercase.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

/// Processes a dictionary file, converting the file to a HashMap of
/// signatures to lists of words.
fn process_dictionary(dict: &mut HashMap<String, Vec<String>>, file: &mut File) {
    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        // I don't like this, but the borrow checker complains otherwise
        let key = l.clone();
        let entry = dict.entry(word_signature(l)).or_insert(Vec::new());
        entry.push(key);
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
        Err(_err) => {
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
        print!("    Word: ");
        match io::stdout().flush().unwrap();

        let mut word = String::new();
        match stdin.read_line(&mut word) {
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

        // Get around a borrow checker complaint
        let w = word.clone();
        let sig = word_signature(word);

        match dict.get(&sig) {
            None => {
                println!("   Error: '{}' not in dictionary.", w);
            }
            Some(list) => {
                println!("Anagrams: {}", list.join(", "));
            }
        }

        println!("");
    }
}
