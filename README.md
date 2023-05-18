# swift_file

Rust implementation of transferring files between devices over Wi-Fi network using a QR code.
Tool is inspired by https://github.com/claudiodangelis/qrcp

## How does it work?

The sf server is bound to the IP address of a default network interface of the machine the server is running on. Alternatively, the IP address (`--port`), particular network interface (`--interface`), and port (`--port`) can be selected by the user.

The QR code that is printed encodes a http URL which is typically of the following format:

`http://{ip}:{port}/{download|receive}/[optional suffix]`

## Known limitations

- Some browsers on iOS are unable to download the file. It always works with Safari but fails with Brave. The failed download might also occur on other Chromium-based iOS browsers.

## Installation options

### Install with cargo

swift_file is published on crates.io (https://crates.io/crates/swift_file) and can be directly installed.
In order to install from crates.io, it is required to have Rust and cargo installed on your system.

```sh
cargo install swift_file
```

### Manual installation from an archive

[Latest release](https://github.com/mateoradman/swift_file/releases/latest) page provides an option to manually install the sf binary from an archive. The archive is available for Linux, MacOS, and Windows.
Download, extract and move the binary to the desired directory, and set execution permissions.

#### Linux

1. Download the linux archive from [Latest release](https://github.com/mateoradman/swift_file/releases/latest)
2. Extract the archive

```sh
tar xf swift_file_*_x86_64-unknown-linux-musl.tar.gz
```

3. Move the binary

```sh
sudo mv sf /usr/local/bin
```

4. Set execution permissions

```sh
sudo chmod +x /usr/local/bin/sf
```

5. Run sf

```sh
sf --help
```

#### MacOS

1. Download the Apple darwin archive from [Latest release](https://github.com/mateoradman/swift_file/releases/latest)
2. Extract the archive

```sh
unzip swift_file_*_x86_64-apple-darwin.zip
```

3. Move the binary

```sh
sudo mv sf /usr/local/bin
```

4. Set execution permissions

```sh
sudo chmod +x /usr/local/bin/sf
```

5. Run sf

```sh
sf --help
```

#### Windows

1. Download the Windows archive from [Latest release](https://github.com/mateoradman/swift_file/releases/latest)
2. Extract the archive
3. Run sf.exe

## CLI Usage

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

### Sending a file to another device

```
Send a file

Usage: sf send [OPTIONS] <FILE>

Arguments:
  <FILE>  File path to send

Options:
      --ip <IP>                IP Address to bind to
  -i, --interface <INTERFACE>  Network interface to use (ignored if --ip provided)
  -p, --port <PORT>            Server port
  -h, --help                   Print help

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
