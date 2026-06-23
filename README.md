# deliminator

A fast CLI tool for splitting and concatenating CSV files, built with Rust.

## Features

- **Split by row count** — divide a CSV into chunks of N rows each
- **Split by file count** — divide a CSV into N evenly-sized files
- **Concatenate** — merge multiple CSVs into one (headers handled automatically)
- Parallel processing via Rayon for large files
- Progress bar feedback

## Installation

```bash
cargo build --release
# binary at ./target/release/deliminator
```

## Usage

```
deliminator <CSV_PATH> [OPTIONS] <COMMAND>
```

### Commands

#### `chunks` — split into files of N rows each

```bash
deliminator data.csv chunks 1000
```

Splits `data.csv` into files of 1000 rows each.

#### `files` — split into N files

```bash
deliminator data.csv files 5
```

Splits `data.csv` into 5 evenly-sized files.

#### `concat` — concatenate CSVs

```bash
deliminator data.csv concat part2.csv part3.csv
```

Merges `data.csv`, `part2.csv`, and `part3.csv` into a single output file.

### Options

| Flag | Description |
|------|-------------|
| `-o, --output <PATH>` | Output directory (default: auto-generated next to input) |

## Output

Output files are written to a directory named after the input file and command (e.g. `data_chunks_1000/`). Each split file is named sequentially: `data_0.csv`, `data_1.csv`, etc.
