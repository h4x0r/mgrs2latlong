# MGRS to Lat/Long Converter

A Rust command-line tool for converting MGRS (Military Grid Reference System) coordinates to latitude/longitude in CSV files.

## Author

Albert Hui <albert@securityronin.com>

## Features

- Automatically detects MGRS coordinate columns in CSV files
- Converts MGRS coordinates to decimal latitude/longitude
- Preserves original data while adding new latitude and longitude columns
- Supports flexible MGRS format recognition
- Command-line interface with input/output file options

## Installation

### Install from crates.io (Recommended)

```bash
cargo install mgrs2latlong
```

### Build from Source

Ensure you have Rust installed, then build the project:

```bash
cargo build --release
```

## Usage

```bash
# Convert MGRS coordinates in input.csv and write to output.csv
mgrs2latlong input.csv --output output.csv

# Convert and output to stdout
mgrs2latlong input.csv

# If built from source, use the full path:
# ./target/release/mgrs2latlong input.csv --output output.csv
```

## Input Format

The tool accepts CSV files with MGRS coordinates in any column. MGRS coordinates should follow the standard format (e.g., "33TWM1234567890" or "33T WM 12345 67890").

## Output Format

The tool outputs a CSV file with:
- All original columns preserved
- Additional `latitude` column with decimal degrees
- Additional `longitude` column with decimal degrees

## Dependencies

- `clap` - Command-line argument parsing
- `csv` - CSV file handling
- `regex` - Pattern matching for MGRS detection
- `geoconvert` - MGRS coordinate conversion
- `anyhow` - Error handling

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.