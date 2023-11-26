use std::{fs, io};

fn main() {
    clear_terminal();
    println!("Add anything to any text file");
    println!("Please enter the file name and extension");
    let mut file = String::new();
    io::stdin().read_line(&mut file).expect("Failed to read line");
    let mut addedtext = String::new();
    println!("Please enter the content you would like to add");
    io::stdin().read_line(&mut addedtext).expect("Failed to read line");
    let mut text = fs::read_to_string(&file.trim()).expect("Unable to read file");
    text.push_str(&addedtext.trim_end());
    println!("{text}");
    fs::write(&file.trim(), &text).expect("Error Writing to file");
    clear_terminal();
    println!("Content added to file");
}

fn clear_terminal() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status().unwrap();
}