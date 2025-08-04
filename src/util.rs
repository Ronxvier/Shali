use std::fs;
use reedline::{DefaultPrompt, Reedline, Signal};
mod operations;
use colored::*;
pub fn engine() {
    processor(readinput());
}

pub fn readinput() -> String {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let sig = line_editor.read_line(&prompt);
    match sig {
        Ok(Signal::Success(buffer)) => {
            return buffer;
        }
        Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
            return String::from("Error");
        }
        x => {
            println!("Event: {:?}", x);
            return String::from("Error");
        }
    }
}

pub fn processor(slawg: String) {
    // matcher function to match inputs to processes
    match slawg.as_str() {
        "add" => operations::add(),
        "edit" => operations::edit(),
        "delete" => operations::delete(),
        "encrypt" => operations::encrypt(),
        "decrypt" => operations::decrypt(),
        "list" => operations::list(),
        "quit" => std::process::exit(0),
        _ => {
            println!("{}","invalid input. As a reminder, valid inputs include 'add', 'edit', 'delete', 'encrypt', 'decrypt', and 'quit'".red());
            processor(readinput());
        },
    }
}

