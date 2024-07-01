#![allow(special_module_name)] // ? Not sure why is throwing warning
mod lib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() == 3 || args.len() == 4) {
        println!("Not enough arguments suplied.");
        return;
    }

    let file_path: String = args[1].clone();
    let separator: char;
    if args[2].len() == 1 {
        separator = args[2].chars().nth(0).unwrap();
    } else {
        println!("Separator must be one character.");
        return;
    }
    let mut enable_header = false;

    if args.len() == 4 && args[3] == "--enable-header" {
        enable_header = true;
    }

    let result = lib::parse(&file_path, enable_header, separator).unwrap();

    if let Some(h) = result.header {
        println!("Header: \n{:?}\n", h);
    }

    println!("Content:");
    for row in result.content {
        println!("{:?}", row);
    }
}
