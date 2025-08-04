use std::fs;
use std::process::Command;
use std::fs::File;
use std::fs::write;
use std::fs::create_dir;
use std::io::Write;
use crate::util::{readinput, processor};
use colored::*;
use simple_crypt::{encrypt_directory,decrypt_directory};
use std::path::{Path, PathBuf};
use std::fs::remove_dir_all;
use std::fs::remove_file;
use std::fs::read_dir;
use colored::*;
use std::{thread, time::Duration};

pub fn add() {
    ensure_dir();
    println!("Name this note:");
    let name = readinput();
    let binding = name.trim();
    if !binding.is_empty() {
        println!("Success! Your new note is called {}", name.blue().bold());
        let output = Command::new("vi")
            .args([format!("shali/{}", name).as_str()])
            .status();

    } else {
        println!("please input a valid name");
        add();
    }
    processor(readinput());
}

pub fn edit() {
    println!("Which file would you like to edit?");
    let filename = readinput();
    let output = Command::new("vi")
        .args([format!("shali/{}", filename).as_str()])
        .status();
    processor(readinput());
}

pub fn delete() {
    println!("Which note do you want to delete?");
    let filename = readinput();
    remove_file(Path::new(format!("shali/{}.txt",filename).as_str()));
    println!("Deleted the specified file.");
    processor(readinput());
}

pub fn list(){
    // directly copied from Kai Sellgren on stackoverflow :/
    let paths = fs::read_dir("shali").unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        std::process::exit(1);
    });
    for path in paths {
        println!("• {}", path.unwrap().path().display())
    }
    processor(readinput());
}

pub fn encrypt() {
    println!("Enter an encryption password:");
    let password = readinput();
    encrypt_directory(Path::new("shali"), Path::new("shali.dir"), password.as_bytes()).expect("Failed to encrypt directory");
    thread::sleep(Duration::from_millis(100));
    remove_dir_all(Path::new("shali"));
    processor(readinput());
}

pub fn decrypt() {
    println!("Enter your encryption password:");
    let password = readinput();
    decrypt_directory(Path::new("shali.dir"), Path::new("."), password.as_bytes()).expect("Failed to decrypt directory");
    thread::sleep(Duration::from_millis(100));
    remove_dir_all(Path::new("shali.dir"));
    processor(readinput());
}
// Helper functions


pub fn ensure_dir() -> std::io::Result<()> {
    // function to check if the shali directory exists, and if not initialize one to prevent
    // errors.
    let notes_dir = Path::new("shali");
    if !notes_dir.exists() {
        create_dir(notes_dir)?;
    }
    Ok(())
}


pub fn writer(name: String, content: String) -> std::io::Result<()> {
    ensure_dir();
    let filepath: PathBuf = Path::new("shali").join(format!("{}.txt", name));
    write(filepath, content)?;
    Ok(())
}
