# RustyUserHub

RustyUserHub is a high-performance, RESTful API crafted with Rust and Actix-web for seamless CRUD operations.

## Features

- **User Management**:
   - Create, read, update, and delete user records.
   - Unique user identification using UUID.
- **Concurrency**:
   - Thread-safe user data management with `Arc` and `Mutex`.
- **RESTful API**:
   - Standard RESTful routes for user operations.

## Technologies Used

- **Rust**: The application is written in Rust.
- **Actix-web**: Used as the web application framework.
- **Serde**: For serialization and deserialization.
- **UUID**: For unique user identification.
- **Arc and Mutex**: For thread-safe data management.

## Core Logic

- **Create User**: Handles user creation with unique ID generation.
- **Get Users**: Retrieves a list of all users.
- **Get User by ID**: Fetches a specific user by their ID.
- **Update User**: Updates user details for a specific ID.
- **Delete User**: Deletes a user by their ID.

<img width="1278" alt="Screenshot 2024-05-23 at 1 32 23 PM" src="https://github.com/shuddha2021/RustyUserHub/assets/81951239/01817017-90bc-4fcb-bbc7-0ac987035fd7">

<img width="1278" alt="Screenshot 2024-05-23 at 1 36 08 PM" src="https://github.com/shuddha2021/RustyUserHub/assets/81951239/9e287e2d-17fc-410b-8d47-6bf93264e101">


## Project Structure

The project consists of the following main files:

- `main.rs`: This is the main Rust file containing the implementation of the RESTful API routes and handlers.
- `Cargo.toml`: This file contains the dependencies and metadata for the Rust project.

## Getting Started

To get started with this project:

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project using `cargo build`.
4. Run the server with `cargo run`.

## Why This Project Is Useful

This project serves as a practical example of implementing a RESTful API in Rust using Actix-web. It demonstrates various concepts such as concurrency, unique identification, and standard RESTful operations in a real-world scenario.

## Contributing

Contributions to this project are welcome. Please fork the repository and create a pull request with your changes.
