# swift_file

Rust implementation of transferring files between devices over Wi-Fi network using a QR code.
Tool is inspired by https://github.com/claudiodangelis/qrcp

## How does it work?

The Axum server is bound to the IP address of a default network interface of the machine the server is running on. IP address and port, or particular network interface, can be selected by providing the `--port`, `--ip` or `--interface` on the command line.

The QR code that is printed to stdout encodes a http URL which is typically of the following format:

`http://{ip}:{port}/{send|receive}/[optional suffix]`

## Current limitations

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

### Sending a file to another device

```
Send or receive files between devices using Wi-Fi network

Usage: sf [OPTIONS] <COMMAND>

Commands:
  send     Send a file
  receive  Receive a file
  help     Print this message or the help of the given subcommand(s)

Options:
      --ip <IP>                IP Address to bind to
  -i, --interface <INTERFACE>  Network interface to use (ignored if --ip provided)
  -p, --port <PORT>            Server port
  -h, --help                   Print help
  -V, --version                Print version
```

### Receiving a file from another device

```
Receive a file

Usage: sf receive [OPTIONS]

Options:
  -d, --dest-dir <DEST_DIR>    Destination directory
      --ip <IP>                IP Address to bind to
  -i, --interface <INTERFACE>  Network interface to use (ignored if --ip provided)
      --no-open                Disable opening the received file automatically using the system default program
  -p, --port <PORT>            Server port
  -h, --help                   Print help
```
