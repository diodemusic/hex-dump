# Hex Dump CLI Tool

Read a file's raw bytes and prints them in a structured hex + optional ASCII table.

---

## 📄 Example

```bash
cargo run -- bee_movie.txt
```

Output:

![Hex dump screenshot](example.png "Hex dump screenshot")

---

## ✨ Features

- **Hex + ASCII display** side-by-side (toggleable).
- **Aligned offsets** for easy navigation (can be hidden).
- **Configurable width** (default: 16 bytes per row).
- **Groups of 8 bytes** separated for clarity.
- **Optional header** showing byte positions.

---

## 📦 Installation

Make sure you have Rust installed ([Install Rust](https://www.rust-lang.org/tools/install)).

Clone this repository:

```bash
git clone https://github.com/diodemusic/hex-dump.git
cd hex-dump
```

---

## ▶ Usage

Basic usage:

```bash
cargo run -- <filename>
```

### Options

| Flag / Option     | Description                             | Default      |
| ----------------- | --------------------------------------- | ------------ |
| `<filename>`      | File to read and dump.                  |              |
| `-w, --width <N>` | Number of bytes per row.                | `16`         |
| `--no-ascii`      | Hide the ASCII column.                  | show ASCII   |
| `--no-header`     | Hide the header row and separator line. | show header  |
| `--no-columns`    | Hide the left-hand offset column.       | show offsets |

---

### Examples

Dump with 8 bytes per row:

```bash
cargo run -- bee_movie.txt --width 8
```

Hex only (no ASCII):

```bash
cargo run -- bee_movie.txt --no-ascii
```

Minimal mode (just hex, no offsets or header):

```bash
cargo run -- bee_movie.txt --no-ascii --no-columns --no-header
```

---
