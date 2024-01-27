# Actix Web Starter Repo

An basic actix web starter repository.

## Basic configuration

An actix-web application, with CLI interface to configure host and port. Includes a rough app structure that separates 
tests from app code inspired by [zero-to-prod](https://github.com/LukeMathWalker/zero-to-production)

### Main dependencies

- [Actix web](https://actix.rs)
- [lexopt](https://docs.rs/lexopt/latest/lexopt/) for CLI interface
- [Askama](https://docs.rs/askama/latest/askama/) for Jinja-like templating
- [Thirtyfour](https://github.com/stevepryde/thirtyfour) for browser end-to-end tests
- GitHub actions that depend on https://github.com/dtolnay/rust-toolchain

## Usage

To use this repository you will need:
- Rust

### Local

To run locally:

1. Run cargo

    ```bash
    cargo run
    ```
    
    Visit http://127.0.0.1:8080 to view the app.
    
    Optionally run on a different port
    
    ```bash
    cargo run -- -p 5000
    ```
    
    View other CLI options with:
    
    ```bash
    cargo run -- --help
    Usage: actix_starter [-h|--host=X.X.X.X] [-p|--port=XXXX]
    ```

### Docker

If you have Docker installed you can build the app with the following commands:

1. Build the image
   ```bash
   docker build . -t actix_starter
   ```

2. Run the image
   ```bash
   docker run --rm -p 8080:8080 actix_starter:latest
   ```

### Testing

To run the full test suite you will need to install and start [geckodriver](https://github.com/mozilla/geckodriver/releases).
An example script for downloading geckodriver for MacOS is included in [drivers](./drivers/get_gecko.sh).

1. Start geckodriver in the background
   ```bash
   geckodriver &
   ```

2. To run the tests:

   ```bash
   cargo test
   ```
