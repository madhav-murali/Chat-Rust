# Chat Service in Rust 😎

This project is a scalable WebSocket-based chat server implemented in Rust using the `warp` web framework and `tokio` for asynchronous runtime. It supports real-time messaging and can handle multiple concurrent users.

## Features

- **WebSocket Communication**: Real-time two-way communication using WebSockets.
- **Multi-User Support**: Maintains a set of active user connections.
- **Message Broadcasting**: Messages sent by a user are broadcast to all connected users.
- **Health Check Endpoint**: Simple health check route to verify server status.

## Prerequisites

Before running the application, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (1.65.0 or newer)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Installation

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd chat-service
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## API Endpoints

### WebSocket Endpoint
- **URL**: `ws://127.0.0.1:3030/chat/<user_id>`
- **Description**: Establishes a WebSocket connection for the specified user.
- **Path Parameter**:
  - `<user_id>`: A unique identifier for the user.

### Health Check Endpoint
- **URL**: `http://127.0.0.1:3030/health`
- **Description**: Returns `OK` to indicate the server is running.

## Code Overview

### Main Components

1. **`main` Function**:
   - Sets up routes for WebSocket connections and health checks.
   - Starts the server on `127.0.0.1:3030`.

2. **WebSocket Handling**:
   - The `handle_connection` function manages individual WebSocket connections, including receiving and broadcasting messages.

3. **Broadcasting Messages**:
   - Messages are broadcast to all connected users using a shared `HashSet` of user channels.

## Example Usage

### Starting the Server

Run the server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:3030`.

### Connecting a Client

Use a WebSocket client (e.g., a browser, [Postman](https://www.postman.com/), or [websocat](https://github.com/vi/websocat)) to connect to the WebSocket endpoint:
```bash
ws://127.0.0.1:3030/chat/<user_id>
```

Replace `<user_id>` with a unique identifier, such as `user123`.

### Health Check

Verify the server status:
```bash
curl http://127.0.0.1:3030/health
```

Expected response:
```
OK
```

## Logging

The server provides useful logs for debugging:
- New connections and disconnections.
- Broadcasted messages.
- Errors during message transmission.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

