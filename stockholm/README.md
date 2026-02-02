# Stockholm

```diff
- This project is for educational purposes only. You should never use this type of program for malicious purposes.
```

Stockholm is a ransomware-like software who can be used to encrypt file recursively from a directory.

## Build
To build Stockholm, you need to have Rust installed on your system. You can then clone the repository and build the project using Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```

## Features

```bash
$ stockholm --help
Usage: stockholm.exe [OPTIONS]

Options:
  -r, --reverse <KEY>  Reverse the infection with the KEY
  -s, --silent         Silent any output
  -h, --help           Print help
  -V, --version        Print version
```
