# notify

A lightweight, cross-platform desktop notification utility written in Rust.

`notify` sends desktop notifications from the command line with a title and an optional body.

Supports custom application names, icons, timeouts, and urgency levels.

On macOS, only the title and body are supported. The `--app-name` (`-a`), `--icon` (`-i`), `--expire-time` (`-e`), and `--urgency` (`-u`) options are unavailable.

## Installation

### Cargo

```sh
cargo install --git https://github.com/yutadev31/notify
```

### Nix

```sh
nix profile install github:yutadev31/notify
```

## Usage

### Send a notification

```sh
notify "Hello, world!"
```

```sh
notify "Build complete" "Your project is ready."
```

### Set an application name

```sh
notify -a "Build script" "Build complete"
```

The application name defaults to `notify`.

### Set an icon

```sh
notify -i dialog-information "Build complete"
```

### Set a timeout

```sh
notify -e 5000 "Reminder" "Take a break."
```

The timeout is specified in milliseconds. The default is `-1`, which uses the notification server's default timeout. Use `0` to request a notification that does not expire automatically.

### Set an urgency level

```sh
notify -u critical "Build failed" "Check the build logs."
```

Available urgency levels are `low`, `normal`, and `critical`. The default is `normal`.

## Platform Support

`notify` uses [notify-rust](https://github.com/hoodie/notify-rust) for desktop notifications.

It supports Linux and other Unix-like systems with a desktop notification service over D-Bus, as well as macOS and Windows.

On other platforms, support for notification options varies by platform and notification server. Icons, timeouts, and urgency levels may be ignored.

## Development

A Nix development environment is available:

```sh
nix develop
```

Run checks with Cargo:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Build the release binary:

```sh
cargo build --release
```

## License

Licensed under the [MIT License](LICENSE.txt).
