# syncd

`syncd` is a small Rust gRPC service for managing todo items. It uses SQLite to store items locally.

## Features

- Add todo items.
- List saved todo items.
- Store data in SQLite.
- Communicate through a gRPC API.

## Tech stack

- Rust
- Tonic
- Protocol Buffers
- SQLx
- SQLite

## Running the project

Start the server:

```bash
cargo run --bin server
```

Keep the server running, then open a second terminal and run the client:

```bash
cargo run --bin client
```

The example client adds a todo item and then retrieves the saved items from the database.

## Database

The application creates a local SQLite database called `syncd.db` when the server starts. The database file is local development data and should not be committed to Git.

## API

The service currently provides these RPC methods:

- `AddItem` adds a new todo item.
- `ListItems` returns saved todo items.
- `CompleteItem` is planned for database integration.
- `DeleteItem` is planned for database integration.

## Project status

Adding and listing items currently work. Database support for completing and deleting items is still in progress.