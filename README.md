# Rust Web Server

This project is a simple web server built with Rust using the Warp web framework. The server is designed to be clean, fast, efficient, and secure, and it can serve both static files and dynamic routes. Additionally, the project includes unit and integration tests to ensure the server's functionality.

## Features

- **Serve Static Files:** The server serves static files from a specified directory (e.g., `./static`).
- **Dynamic Routes:** Supports dynamic routes, such as an "About" page.
- **Error Handling:** Custom error handling for 404 (Not Found) and 500 (Internal Server Error) responses.
- **Logging:** Logs requests and errors using the `tracing` crate.
- **Testing:** Includes unit and integration tests to ensure the server behaves as expected.

## Project Structure

```plaintext
.
├── src
│   ├── lib.rs           # Core logic of the web server
│   └── main.rs          # Entry point of the application
├── static               # Directory for static files (e.g., index.html)
├── tests
│   └── integration_test.rs  # Integration tests for the web server
├── Cargo.toml           # Project dependencies and configuration
└── README.md            # Project documentation (this file)
```

### `src/lib.rs`

Contains the core logic for the web server, including route definitions and error handling.

### `src/main.rs`

The entry point of the application, where the web server is initialized and run.

### `static/`

Directory for static files that are served by the web server (e.g., `index.html`).

### `tests/integration_test.rs`

Contains integration tests to verify that the server's routes are working correctly.

## How to Use

### Prerequisites

- **Rust**: Ensure that you have Rust installed. You can install it via `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Running the Web Server

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd rust_web_server
   ```

2. **Build and Run the Server**:
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:3030`. You can access it in your web browser.

3. **Access the Server**:
   - **Root Path (`/`)**: Serves the `index.html` file from the `./static` directory.
   - **About Page (`/about`)**: Returns a simple "This is the about page" response.
   - **404 Not Found**: Any undefined route will return a custom 404 response.

### Running Tests

To run the unit and integration tests, use the following command:

```bash
cargo test
```

This will execute all tests in the project and report the results.

### Extending the Server

- **Add New Routes**: You can define additional routes in the `get_routes()` function in `src/lib.rs`.
- **Serve More Static Files**: Place additional static files in the `./static` directory. The server will automatically serve them.
- **Customize Error Handling**: Modify the `handle_rejection` function in `src/lib.rs` to customize how errors are handled.

## Future Improvements

- **HTTPS Support**: Add HTTPS support for secure connections.
- **Advanced Routing**: Implement more complex routing and request handling (e.g., query parameters, POST requests).
- **Deployment**: Deploy the server to a cloud platform (e.g., AWS, DigitalOcean) for public access.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---
