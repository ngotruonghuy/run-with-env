## run-with-env

`run-with-env` is a Rust command-line utility designed to run executables with environment variables loaded from a `.env` file. It is useful for local development and scripting, allowing you to easily inject environment variables into any process.

### Features
- Loads environment variables from a `.env` file
- Runs any executable with the loaded environment
- Simple and fast, with minimal dependencies

## Installation

You can build the project from source using Cargo:

```sh
cargo build --release
```

The compiled binary will be located in `target/release/exec.exe` (on Windows).

## Usage

Place a `.env` file in your working directory with the desired environment variables, for example:

```
API_KEY=your_api_key
DEBUG=true
```

Then run your executable with environment variables loaded:

```sh
exec.exe <your_command> [args...]
```

Example:

```sh
exec.exe cargo run
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Pull requests and issues are welcome! Please open an issue to discuss your ideas or report bugs.
