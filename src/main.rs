use colored::*;
use std::{env, fs, process};

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run -- <filename>");
        process::exit(1);
    }

    let filename: String = args[1].to_string();

    filename
}

fn get_bytes(filename: String) -> std::io::Result<Vec<u8>> {
    let bytes: Vec<u8> = fs::read(filename)?;
    Ok(bytes)
}

fn get_header() {
    let mut count: u8 = 0;
    print!("          ");
    for i in 0..16 {
        print!("{} ", format!("{:02x}", i).cyan());

        if count == 7 {
            print!(" ");
        }

        count += 1;
    }
    println!();
    print!("          {}", "-".repeat(23).blue());
    println!("  {}", "-".repeat(23).blue());
}

fn hex_dump(bytes: Vec<u8>) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let mut count: i32 = 0;

        print!("{}", format!("{:08x} ", i * 16).cyan());

        for &b in chunk {
            if b.is_ascii_graphic() {
                print!("{}", format!(" {:02x}", b).green());
            } else if b == b' ' {
                print!("{}", format!(" {:02x}", b).dimmed());
            } else {
                print!("{}", format!(" {:02x}", b).red());
            }

            if count == 7 {
                print!(" ");
            }

            count += 1;
        }

        if chunk.len() < 16 {
            if chunk.len() < 8 {
                print!(" ");
            }
            for _ in 0..16 - chunk.len() {
                print!("   ");
            }
        }

        print!("{}", format!("  | ").blue());
        for &b in chunk {
            if b.is_ascii_graphic() {
                print!("{}", format!("{}", b as char).green());
            } else if b == b' ' {
                print!("{}", ".".dimmed());
            } else {
                print!("{}", ".".red());
            }
        }

        if chunk.len() < 16 {
            for _ in 0..16 - chunk.len() {
                print!(" ");
            }
        }

        println!("{}", format!(" |").blue());
    }

    println!("Read {} bytes.", bytes.len());
}

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let filename: String = get_filename();
    let bytes: Vec<u8> = get_bytes(filename).expect("Error.");
    get_header();
    hex_dump(bytes);
    println!("Finished in {:.2?}", start.elapsed());
}
