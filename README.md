# swift_file

Rust implementation of transferring files between devices over Wi-Fi network using a QR code.
Tool is inspired by https://github.com/claudiodangelis/qrcp

# How does it work?

The Axum server is bound to the local IP address of the machine the server is running on. The available port can be selected by providing the `--port` on the command line or it will be allocated automatically.

The QR code that is printed to stdout encodes a http url which is typically of the following format:
`http://{ip}:{port}/{send|receive}{optional suffix}`
