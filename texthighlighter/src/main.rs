use std::fs;
use std::io;

fn main() {
    let file_path = "log_sample.txt";

    println!("Reading from file: {}", file_path);

    println!("There are 4 type of log level");

    println!("[ERROR]");
    println!("[DEBUG]");
    println!("[WARNING]");
    println!("[INFO]");
    
    println!("Please input your log level.");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let log_type: String = user_input.trim().parse().expect("Please type a correct log type!");

    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    let highlighted = highlight_content(&content, &log_type);
    println!("File contents: \n{}", highlighted);
}

fn highlight_content(content: &String, log_level: &String) -> String {
    let color = get_log_color(&log_level);
    content.replace(log_level,  &format!("{color}{}{}", log_level, "\x1b[0m"))
}

fn get_log_color(level: &str) -> &str {
    match level {
        "ERROR" => "\x1b[31m",   // red
        "WARNING" => "\x1b[33m", // yellow
        "INFO" => "\x1b[32m",    // green
        "DEBUG" => "\x1b[34m",   // blue
        _ => "\x1b[0m",          // default (no color)
    }
}