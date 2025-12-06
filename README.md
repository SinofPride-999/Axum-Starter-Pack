# Axum Starter Pack

A minimal, production-ready Axum backend template with a modular folder structure. Clone it and start building Rust APIs immediately.

---

## Features

* Pre-configured project structure
* Modular routing and controller setup
* JSON request/response handling
* Example controller with sample endpoints
* Placeholder folders for models and middleware
* Pre-installed dependencies for Axum 0.8.7 and Tokio

---

## Project Structure

```
src/
 ├── main.rs          # Entry point for the application
 ├── lib.rs           # Creates and returns the router
 ├── routes/
 │     └── mod.rs     # Defines all routes
 ├── controller/
 │     └── mod.rs     # Request handlers / controllers
 ├── model/
 │     └── mod.rs     # Data structures for requests/responses
 └── middleware/
       └── mod.rs     # Placeholder for middleware
```

---

## Quick Start

1. Clone the repository:

```bash
git clone git@github.com:SinofPride-999/Axum-Starter-Pack.git
cd Axum-Starter-Pack
```

2. Run the application:

```bash
cargo run
```

3. The server will start on:

```
http://localhost:3000
```

4. Test the routes:

```
GET /             → Returns a welcome message
GET /hello        → Returns "Hello from Axum!"
```

---

## Dependencies

* axum = "0.8.7"
* tokio = { version = "1", features = ["full"] }
* serde = { version = "1", features = ["derive"] }
* serde_json = "1"
* tower = "0.5"

---

## Usage

* Add new routes in `routes/mod.rs`
* Create request handlers in `controller/mod.rs`
* Define data structures in `model/mod.rs`
* Add middleware in `middleware/mod.rs`

This template is ideal for quickly starting Rust API projects and can be extended with JWT authentication, database integration, or additional features as needed.

---

## License

This project is licensed under the MIT License.
