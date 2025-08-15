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

fn hex_dump(bytes: Vec<u8>) {
    println!("Reading {} bytes...", bytes.len());

    let mut body = String::new();

    // Header
    body.push_str("          ");
    for i in 0..16 {
        body.push_str(&format!("{:02x} ", i));
        if i == 7 {
            body.push(' ');
        }
    }
    body.push('\n');
    body.push_str("          ");
    body.push_str(&"-".repeat(23));
    body.push_str("  ");
    body.push_str(&"-".repeat(23));
    body.push('\n');

    // Body
    for (i, chunk) in bytes.chunks(16).enumerate() {
        body.push_str(&format!("{:08x} ", i * 16));

        for j in 0..16 {
            if j < chunk.len() {
                body.push_str(&format!(" {:02x}", chunk[j]));
            } else {
                body.push_str("   ");
            }
            if j == 7 {
                body.push(' ');
            }
        }

        body.push_str("  | ");
        for &b in chunk {
            let c = if b.is_ascii_graphic() { b as char } else { '.' };
            body.push(c);
        }
        for _ in chunk.len()..16 {
            body.push(' ');
        }
        body.push_str(" |\n");
    }

    print!("{}", body);
    println!("Read {} bytes.", bytes.len());
}

fn main() {
    let start: std::time::Instant = std::time::Instant::now();
    let filename: String = get_filename();
    let bytes: Vec<u8> = get_bytes(filename).expect("Error.");
    hex_dump(bytes);
    println!("Finished in {:.2?}.", start.elapsed());
}
