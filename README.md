# Ishango

A simple command-line tool for tracking numerical values in different buckets.

## Installation

Requires Rust and Cargo. Build from source:

```bash
git clone <repository-url>
cd ishango
cargo build --release

The binary will be available at target/release/ishango
Usage
Initialize a bucket

bash

ishango init <bucket-name>

Creates a new bucket. Bucket names can only contain letters, numbers, hyphens, and underscores.

Example:

bash

ishango init my-savings

Add a transaction

bash

ishango add <bucket-name> <value>

Adds a value to the specified bucket. Values can be positive or negative decimals.

Examples:

bash

ishango add my-savings 100.50
ishango add my-savings -25.75

Check balance

bash

ishango balance <bucket-name>

Shows the sum of all transactions in the bucket.

Example:

bash

ishango balance my-savings

View transactions

bash

ishango transactions <bucket-name>

Lists all transactions with their timestamps.

Example:

bash

ishango transactions my-savings

List buckets

bash

ishango list

Shows all existing buckets.
Data Storage

All data is stored in JSONL files in your local data directory:

    Linux: ~/.local/share/ishango/
    macOS: ~/Library/Application Support/ishango/
    Windows: %APPDATA%\ishango\
