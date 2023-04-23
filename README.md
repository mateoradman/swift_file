# swift_file

Rust implementation of transferring files between devices over Wi-Fi network using a QR code.
Tool is inspired by https://github.com/claudiodangelis/qrcp

## How does it work?

The Axum server is bound to the local IP address of the machine the server is running on. The available port can be selected by providing the `--port` on the command line or it will be allocated automatically.

The QR code that is printed to stdout encodes a http url which is typically of the following format:

`http://{ip}:{port}/{send|receive}/[optional suffix]`

## Current limitations

- The maximum data that can be sent with each request is 1GB which should satisfy most users.
- Some browsers on iOS are unable to download the file. It always works with Safari but fails with Brave. The failed download might also occur on other Chromium-based iOS browsers.

## Installation options

### Install with cargo

swift_file is published on crates.io (https://crates.io/crates/swift_file) and can be directly installed.

```sh
cargo install swift_file
```

### Manual installation from an archive

[Releases](https://github.com/mateoradman/swift_file/releases) page provides an option to manually install the sf binary from an archive. The archive is available for Linux, MacOS, and Windows.
Download, extract and move the binary to the desired directory, and set execution permissions.

## CLI Usage

```
Usage: sf <COMMAND>

Commands:
  send     Send a file
  receive  Receive a file
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Sending a file to another device

```
Send a file

Usage: sf send [OPTIONS] <FILE>

Arguments:
  <FILE>  File path to send

Options:
  -p, --port <PORT>  Port to bind the server to (allowed user port range 1024 to 49151)
  -h, --help         Print help
```

### Receiving a file from another device

```
Receive a file

Usage: sf receive [OPTIONS]

Options:
  -p, --port <PORT>  Port to bind the server to (allowed user port range 1024 to 49151)
  -h, --help         Print help
```
