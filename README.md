# HTTP Status Codes (Rust)

A simple Rust CLI tool that displays HTTP status codes in a formatted table.

## Features

- Displays all standard HTTP status codes (1xx-5xx)
- Color-coded output for better readability
- Comprehensive test suite
- Uses `comfy_table` for beautiful table formatting

## Installation

Make sure you have Rust installed on your system. Then clone and build:

```bash
git clone git@github.com:beto-codes/httpstatus-rust.git
cd httpstatus-rust
cargo build --release
```

## Usage

Run the application to display all HTTP status codes:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/httpstatus
```

## Output

The tool displays a formatted table with:

- **Code** - The numeric HTTP status code (in red)
- **Description** - The human-readable description (in green)

## Status Code Categories

- **1xx** - Informational responses (4 codes)
- **2xx** - Success (10 codes)
- **3xx** - Redirection (9 codes)
- **4xx** - Client errors (29 codes)
- **5xx** - Server errors (11 codes)

Total: 63 HTTP status codes

## Testing

Run the test suite:

```bash
cargo test
```

The tests verify:

- Correct number of status codes
- Proper ordering (ascending)
- Existence of common codes (200, 404, etc.)
- Distribution across categories
- Non-empty descriptions

## Dependencies

- `comfy_table` - For terminal table formatting with colors
