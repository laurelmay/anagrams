use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::iter::FromIterator;

pub type AnagramMatch = HashSet<String>;
pub type Dictionary = HashMap<String, AnagramMatch>;

pub trait DictionaryMethods {
    fn from_file(path: &str) -> io::Result<Dictionary>;
    fn update(&mut self, path: &str) -> io::Result<()>;
    fn lookup(&self, word: &str) -> Option<&AnagramMatch>;
}

impl DictionaryMethods for Dictionary {
    fn from_file(path: &str) -> io::Result<Dictionary> {
        let mut dictionary = Dictionary::new();
        dictionary.update(path)?;
        Ok(dictionary)
    }

    fn update(&mut self, path: &str) -> io::Result<()> {
        println!("Processing word list... [{}]", path);
        let words_file = fs::read_to_string(path)?;
        let words = parse_words(&words_file);
        process_dictionary(self, words);
        println!();
        Ok(())
    }

    fn lookup(&self, word: &str) -> Option<&AnagramMatch> {
        self.get(&word_signature(word))
    }
}

/// Processes a dictionary file, converting the file to a HashMap of
/// signatures to lists of words.
fn process_dictionary(dict: &mut Dictionary, words: Vec<&str>) {
    for word in words {
        dict.entry(word_signature(word))
            .or_insert_with(HashSet::new)
            .insert(word.to_string());
    }
    dict.retain(|_, words| words.len() != 1);
}

/// Common way to get the signature of a particular word.
fn word_signature(word: &str) -> String {
    let lowercase = word.to_lowercase();
    let mut chars: Vec<char> = lowercase.chars().collect();
    // sort the letters within the word, allowing all words containing the
    // same letters to have the same signature
    chars.sort_unstable();
    let chars = chars;
    String::from_iter(chars)
}

fn parse_words(dictionary_contents: &str) -> Vec<&str> {
    dictionary_contents
        .lines()
        .filter(|word| !word.trim().is_empty())
        .collect()
}

#[cfg(test)]
#[test]
fn word_sig_test() {
    assert_eq!(word_signature("test"), "estt");
    assert_eq!(word_signature(""), "");
    assert_eq!(word_signature("a"), "a");
    assert_eq!(word_signature("431"), "134");
    assert_eq!(word_signature("stop"), "opst");
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
