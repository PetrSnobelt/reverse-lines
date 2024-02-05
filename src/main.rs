use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file name and optionally also  output file name as command-line arguments.");
        std::process::exit(1);
    }
    let input_filename = &args[1];
    let output_filename;
    if args.len() < 3 { 
        output_filename = args[2].clone(); 
    }
    else {
        let mut s = input_filename.clone();
        s.push_str("_reversed");
        output_filename = s;
    }

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let reversed: Vec<&str> = contents.lines().rev().collect();
    let output = reversed.join("\n");
    
    fs::write(output_filename, output)
        .expect("Something went wrong writing the file");
}