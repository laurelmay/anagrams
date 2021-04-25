mod dictionary;
mod errors;

extern crate dirs;
extern crate rustyline;

use std::env;
use std::path::PathBuf;

use rustyline::Editor;

use dictionary::{Dictionary, DictionaryMethods};
use errors::CommandError;

fn process_command(arg_str: &str, dictionary: &mut Dictionary) -> Result<(), CommandError> {
    let mut args = arg_str.split_whitespace();
    let cmd = args.next().unwrap_or("/");
    let arg = args.collect::<Vec<&str>>().join(" ");
    match cmd {
        "/help" => {
            println!("Enter a word and all anagrams will be printed.");
            println!("Commands:");
            println!("  WORD        Print anagrams for WORD.");
            println!("  /countsigs  Dump the count of signatures");
            println!("  /dump       Print the entire dictionary (signatures and words).");
            println!("  /load FILE  Load words from FILE");
            println!("  /reset      Delete all entries from the dictionary");
            println!("  /help       Print this help text.");
            println!("  /quit       Exit the program.");
        }
        "/dump" => println!("Full dictionary:\n{:?}", dictionary),
        "/countsigs" => println!("Count: {}", dictionary.keys().len()),
        "/load" => dictionary.update(&arg)?,
        "/reset" => dictionary.clear(),
        "/exit" | "/quit" | "/bye" => return Err(CommandError::ExitCommand),
        _ => println!("Unsupported command: {}", cmd),
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

fn find_matches(word: &str, dict: &Dictionary) {
    if let Some(matches) = dict.lookup(word) {
        let mut match_list = matches.iter().map(|s| &**s).collect::<Vec<&str>>();
        match_list.sort_unstable();
        println!("Anagrams: {}", match_list.join(", "));
    } else {
        eprintln!("No angrams found for '{}'.", word);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
        return;
    }

    let word_list_path = &args[1];
    let mut dict = Dictionary::from_file(word_list_path).expect("Unable to load word list");

    let history_path = determine_history_path();
    let history_path = history_path.as_path();

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_path);

    loop {
        match rl.readline("Word: ") {
            Ok(word) if word.trim().is_empty() => continue,
            Ok(cmd) if cmd.starts_with('/') => match process_command(cmd.trim(), &mut dict) {
                Err(CommandError::IoError(_)) => eprintln!("Unable to load word list"),
                Err(CommandError::ExitCommand) => break,
                Ok(_) => continue,
            },
            Ok(word) => {
                find_matches(word.trim(), &dict);
                rl.add_history_entry(word);
            }
            Err(_) => {
                break;
            }
        }
        println!();
    }
    let _ = rl.save_history(history_path);
}
