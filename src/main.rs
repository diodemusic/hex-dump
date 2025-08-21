use clap::Parser;
use std::{fs, io};

#[derive(Parser)]
#[command(name = "dump", about = "Dump the contents of a file as hex bytes")]
struct Cli {
    filename: String,
    #[arg(long, short, default_value_t = 16)]
    width: usize,

    #[arg(long, short)]
    limit: usize,

    #[arg(long, name = "no-ascii")]
    no_ascii: bool,

    #[arg(long, name = "no-header")]
    no_header: bool,

    #[arg(long, name = "no-columns")]
    no_columns: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let start: std::time::Instant = std::time::Instant::now();
    let bytes: Vec<u8> = fs::read(cli.filename)?;

    if !cli.no_header {
        let mut header = String::from("          ");

        for i in 0..cli.width {
            header.push_str(&format!("{:02x} ", i));

            if i % 8 == 7 {
                header.push(' ');
            }
        }

        header.push_str("\n          ");

        for i in 0..cli.width {
            header.push_str("---");

            if i % 8 == 7 {
                header.push(' ');
            }
        }

        println!("{}", header);
    }

    for (i, chunk) in bytes.chunks(cli.width).enumerate() {
        let mut body = String::new();

        if !cli.no_columns {
            body.push_str(&format!("{:08x}: ", i * cli.width));
        }

        for j in 0..cli.width {
            if j < chunk.len() {
                body.push_str(&format!("{:02x} ", chunk[j]));
            } else {
                body.push_str("   ");
            }

            if j % 8 == 7 {
                body.push(' ');
            }
        }

        if !cli.no_ascii {
            body.push_str(" | ");

            for &b in chunk {
                let c: char = if b.is_ascii_graphic() {
                    b as char
                } else if b == b' ' {
                    ' '
                } else {
                    '.'
                };
                body.push(c);
            }

            for _ in chunk.len()..cli.width {
                body.push(' ');
            }

            body.push_str(" |");
        }

        println!("{}", body);
    }

    println!("Read {} bytes.", bytes.len());
    println!("Finished in {:.2?}.", start.elapsed());

    Ok(())
}
