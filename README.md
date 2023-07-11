# keylogger

![crates.io](https://img.shields.io/crates/v/keylogger.svg)

Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes

## Install

If you have Rust: `cargo install keylogger`

## Usage

```bash
Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes

Usage: keylogger [PATH]

Arguments:
  [PATH]  [default: .keylogger]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Build deb

```sh
cargo install cargo-deb
cargo deb
```
