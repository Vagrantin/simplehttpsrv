Here is the updated `README.md` file with the license set to GPL:

```markdown
# Simple HTTP Server for MPEG-DASH

This is a simple HTTP server implemented in Rust using the `hyper` crate. The server is capable of serving static files, including `.mpd` files for MPEG-DASH streaming, with the correct MIME type.

## Features

- Serves static files from a specified directory.
- Supports serving `.mpd` files with the `application/dash+xml` MIME type.
- Easy to configure and run.

## Prerequisites

- Rust and Cargo installed on your system. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/vagrantin/simplehttpsrv.git
cd simplehttpsrv
```

### 2. Build the Project

```sh
cargo build --release
```

### 3. Run the Server

```sh
cargo run
```

By default, the server will listen on `http://127.0.0.1:3000`.

### 4. Place Your Static Files

Place your static files, including `.mpd` files, in the `./static` directory. You can change the directory by modifying the `base_path` variable in the `main.rs` file.

## Configuration

### Base Path

The `base_path` variable in the `main.rs` file specifies the directory from which the server will serve static files. By default, it is set to `./static`. You can change this to any directory you prefer.

```rust
let base_path = Arc::new(PathBuf::from("./static")); // Change this to your static files directory
```

### MIME Types

The server supports serving the following MIME types based on file extensions:

- `.mpd`: `application/dash+xml`
- `.html`, `.htm`: `text/html`
- `.css`: `text/css`
- `.js`: `application/javascript`
- Default: `application/octet-stream`

You can add more MIME types by modifying the `handle_request` function in the `main.rs` file.

## Example

1. Place an `.mpd` file in the `./static` directory.
2. Run the server using `cargo run`.
3. Access the `.mpd` file via `http://127.0.0.1:3000/yourfile.mpd`.

## License

This project is licensed under the GNU General Public License (GPL). See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgments

- Thanks to the `hyper` crate for making it easy to create HTTP servers in Rust.

```

### Explanation:
- **License**: Updated to specify that the project is licensed under the GNU General Public License (GPL).

You can customize this `README.md` file further based on your specific needs and project details.
