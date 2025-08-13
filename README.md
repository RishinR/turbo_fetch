# Turbo Fetch

Turbo Fetch is a high-performance Rust command-line tool for downloading multiple files from a list of URLs. It demonstrates both asynchronous (concurrent) and sequential file downloading, highlighting the performance benefits of async Rust.

## Features
- **Concurrent (Async) Downloads:** Download many files at the same time using async/await and the Tokio runtime.
- **Sequential Downloads:** Optionally download files one-by-one for comparison.
- **Automatic Directory Creation:** Ensures the `downloads` folder exists before saving files.
- **Flexible Link Format:** Reads a `links.txt` file where each line contains a file name and a URL, separated by a space.
- **Performance Timing:** Prints the total execution time for all downloads.

## Usage

### 1. Prepare Your Links File
Create a `links.txt` file in the project root. Each line should be:

```
filename.ext https://example.com/fileurl
```

Example:
```
file1.pdf https://example.com/file1.pdf
file2.jpg https://example.com/file2.jpg
```

### 2. Build the Project

```
cargo build --release
```

### 3. Run Concurrent (Async) Downloads

This is the default mode (see `main.rs`). It will download all files in parallel and print the total time taken:

```
cargo run --release
```

### 4. Run Sequential Downloads (Optional)

Uncomment the sequential download code in `main.rs` and comment out the async block. Then run:

```
cargo run --release
```

## How Async Improves Performance

With async Rust and the Tokio runtime, Turbo Fetch can download many files at the same time without blocking the main thread. This means network requests are handled concurrently, so the total download time is often less than half of what it would be if files were downloaded one-by-one. For example, downloading 10 files sequentially might take 50 seconds, but with async, it could take under 25 seconds or even less, depending on network speed and server response times.

## Requirements
- Rust (edition 2021 or later)
- Internet connection

## Dependencies
- [tokio](https://crates.io/crates/tokio) (for async runtime)
- [reqwest](https://crates.io/crates/reqwest) (for HTTP requests)
- [futures](https://crates.io/crates/futures) (for join_all)

## License
MIT
