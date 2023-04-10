use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file: File = File::open(filename).expect("File not found");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    // Get the first and second arguments with error catching
    let filename: &str = args.get(2).expect("No filename provided");
    let search_str: &str = args.get(3).expect("No search string provided");

    println!("Filename: {}\nSearch String: {}", filename, search_str);
    let lines: Vec<String> = lines_from_file(filename);
    for (index, line) in lines.iter().enumerate() {
        if line == search_str {
            println!("Found: {} at line index: #{}", search_str, index);
        }
    }
}
