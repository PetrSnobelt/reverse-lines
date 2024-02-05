use core::str;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file name and optionally also  output file name as command-line arguments.");
        std::process::exit(1);
    }
    dbg!(&args);
    let input_filename = &args[1];
    let contents = fs::read_to_string(input_filename)
            .expect("Something went wrong reading the file");

    let output_filename = if args.len() > 2 {
        args[2].clone()
    } else {
        new_name(input_filename)
    };

    let reversed: Vec<&str> = contents.lines().rev().collect();
    let output = reversed.join("\n");

    fs::write(&output_filename, output)
        .expect("Something went wrong writing the file");

    println!("Reversed content written to {}", output_filename);
}

fn new_name(file_path: &str) -> String {
    let suffix = "_reversed";
    let path = Path::new(file_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();
    let new_file_name = format!("{}{}.{}", stem, suffix, extension);
    let new_path = path.with_file_name(new_file_name);
    new_path.to_str().unwrap().to_string()
}

