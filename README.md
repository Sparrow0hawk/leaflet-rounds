# Leaflet Rounds

Find the streets in a series of drawings on a map.

## Usage

To use this repository you will need:
- Rust
- Node and NPM

### Local

To run locally:

1. Build assets
   ```bash
   npm install && npm run build
   ```

2. Run cargo

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
    Usage: leaflet_rounds [-h|--host=X.X.X.X] [-p|--port=XXXX]
    ```

### Docker

If you have Docker installed you can build the app with the following commands:

1. Build assets
   ```bash
   npm install && npm run build
   ```

2. Build the image
   ```bash
   docker build . -t leaflet-rounds
   ```

3. Run the image
   ```bash
   docker run --rm -p 8080:8080 leaflet-rounds:latest
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
