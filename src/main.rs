use std::env;
use std::fs::File;
use std::io::prelude::*;
mod caesar;

fn help() {
    println!("Please enter a valid cipher type and filename");
}

fn read_file(filename: &String) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading from the file");

    return contents;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let cmd = &args[1];
            let filename = &args[2];

            let cipher_text = read_file(filename);

            match &cmd[..] {
                "caesar" => println!("{}", caesar::solve(cipher_text).1),
                _ => help()
            }
        },
        _ => {
            help();
        }
    }
}
