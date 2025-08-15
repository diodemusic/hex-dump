# Hex Dump CLI Tool

A simple hex dump CLI tool written in Rust.  
It reads a file's raw bytes and prints them in a structured hex + ASCII table.

---

## ✨ Features

- **Hex + ASCII display** side-by-side.
- **Aligned offsets** for easy navigation.
- **Groups of 8 bytes** separated for clarity.

---

## 📦 Installation

Make sure you have Rust installed ([Install Rust](https://www.rust-lang.org/tools/install)).

Clone this repository and install dependencies:

```bash
git clone https://github.com/diodemusic/hex-dump.git
cd hex-dump
cargo build
````

---

## ▶ Usage

Run with:

```bash
cargo run -- <filename>
```

Example:

```bash
cargo run -- bee_movie.txt
```

Output:

![Hex dump screenshot](example.png "Hex dump screenshot")

---
