# GEI - GoMaluum Entries Indexer

A simple gRPC-based schedule indexer with built-in encryption and SQLite storage. This service allows you to securely store and retrieve user schedules (JSON data) with automatic encryption/decryption.

## Features

- 🚀 **Fast gRPC API** - Built with Tonic for high-performance communication
- 🔐 **Secure Encryption** - AES-256-GCM encryption for all stored data
- 💾 **SQLite Storage** - Simple, reliable persistence
- 📝 **JSON Support** - Store complex schedule data as JSON
- 🔑 **Admin Authentication** - Protect write operations with admin key

## Architecture

```
                   gRPC API
                      │
        ┌─────────────┼─────────────┐
        │             │             │
   StoreSchedule  GetSchedule    Server
        │             │             │
        └─────────────┼─────────────┘
                      │
                  Encryption
                      │
                   SQLite DB
```

## Prerequisites

- Rust 1.70+ (2021 edition)
- SQLite3 (usually pre-installed on most systems)

## Installation

Clone the repository and build:

```bash
git clone <your-repo-url>
cd gei
cargo build --release
```

## Quick Start

### 1. Configure Environment

Create a `.env` file (or copy `.env.example`):

```env
DATABASE_URL=sqlite://schedules.db
ENCRYPTION_KEY=your-encryption-key-here
ADMIN_KEY=your-admin-key-here
```

Generate a secure admin key:
```bash
openssl rand -base64 32
```

### 2. Start the Server

```bash
cargo run --bin gei-server
```

The server will start on `0.0.0.0:50053` and create a `schedules.db` file in the current directory.

### Using the gRPC API

You can interact with the server using any gRPC client. Here are examples using different tools:

#### Using grpcurl

**Store a schedule (requires admin key):**
```bash
grpcurl -plaintext \
  -H "admin-key: your-admin-key" \
  -d '{
    "username": "alice",
    "schedule_json": "{\"monday\":\"Math 9AM\",\"tuesday\":\"Physics 10AM\"}"
  }' localhost:50053 schedule.ScheduleIndexer/StoreSchedule
```

**Retrieve a schedule (no auth required):**
```bash
grpcurl -plaintext -d '{
  "username": "alice"
}' localhost:50053 schedule.ScheduleIndexer/GetSchedule
```

#### Using your own client

See `proto/schedule.proto` for the service definition and implement your client in any language that supports gRPC.

## Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | SQLite database path | `sqlite://schedules.db` | No |
| `ENCRYPTION_KEY` | Custom encryption key | Auto-generated | No |
| `ADMIN_KEY` | Admin key for write operations | None | Yes* |

*Required for `StoreSchedule` operations

### Examples

**Custom database location:**
```bash
DATABASE_URL="sqlite:///var/lib/gei/schedules.db" cargo run --bin gei-server
```

**Custom encryption key:**
```bash
ENCRYPTION_KEY="my-super-secret-key" cargo run --bin gei-server
```

## API Reference

### gRPC Service Definition

```protobuf
service ScheduleIndexer {
  rpc StoreSchedule(StoreScheduleRequest) returns (StoreScheduleResponse);
  rpc GetSchedule(GetScheduleRequest) returns (GetScheduleResponse);
}
```

### StoreSchedule

Stores or updates a user's schedule with automatic encryption.

**Authentication:** Requires `admin-key` in request metadata.

**Request:**
- `username` (string): Unique identifier for the user
- `schedule_json` (string): Schedule data in JSON format

**Metadata:**
- `admin-key` (string): Admin authentication key

**Response:**
- `success` (bool): Operation status
- `message` (string): Success/error message

**Errors:**
- `UNAUTHENTICATED`: Admin key not provided
- `PERMISSION_DENIED`: Invalid admin key

### GetSchedule

Retrieves and decrypts a user's schedule.

**Authentication:** None required (public read access).

**Request:**
- `username` (string): Username to retrieve

**Response:**
- `success` (bool): Operation status
- `schedule_json` (string): Decrypted schedule data
- `message` (string): Error message if failed

## Security

### Admin Authentication

Write operations (`StoreSchedule`) require admin authentication to prevent unauthorized database modifications:

- Admin key must be sent in the `admin-key` metadata header
- Read operations (`GetSchedule`) remain publicly accessible
- See `QUICKSTART_ADMIN.md` for quick setup guide
- See `ADMIN_AUTH.md` for complete documentation and examples

**Quick example:**
```bash
# Test with example client
export ADMIN_KEY="your-admin-key"
cargo run --example client_with_admin

# Or run the test script
./test_admin_auth.sh
```

### Encryption

- **Algorithm:** AES-256-GCM
- **Nonce:** Random 12-byte nonce per encryption
- **Storage:** Nonce prepended to ciphertext

### Best Practices

1. **Always set a custom encryption key in production:**
   ```bash
   export ENCRYPTION_KEY="your-secure-random-key"
   ```

2. **Always set a strong admin key:**
   ```bash
   export ADMIN_KEY="$(openssl rand -base64 32)"
   ```

3. **Protect your database file:**
   ```bash
   chmod 600 schedules.db
   ```

4. **Use TLS for gRPC in production**

5. **Rotate encryption keys periodically**

6. **Never commit `.env` to version control**

## Development

### Project Structure

```
gei/
├── proto/
│   └── schedule.proto         # gRPC service definition
├── src/
│   ├── crypto.rs              # Encryption/decryption logic
│   ├── db.rs                  # SQLite database operations
│   ├── service.rs             # gRPC service implementation
│   ├── server.rs              # Server binary
│   └── lib.rs                 # Library module
├── examples/
│   └── client_with_admin.rs   # Example client with admin auth
├── build.rs                   # Protobuf compilation
├── Cargo.toml                 # Dependencies
├── ADMIN_AUTH.md              # Admin auth documentation
├── QUICKSTART_ADMIN.md        # Quick setup guide
├── test_admin_auth.sh         # Test script for admin auth
└── README.md                  # This file
```

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

Binary will be in `target/release/gei-server`

## Docker Deployment

```bash
# Build the image
docker build -t gei-server .

# Run the container
docker run -p 50053:50053 -e ENCRYPTION_KEY="your-key" gei-server
```

Or use docker-compose:

```bash
docker-compose up -d
```

## Example Schedule JSON

```json
{
  "monday": [
    {
      "time": "09:00-10:30",
      "class": "Data Structures",
      "room": "A-301"
    }
  ],
  "tuesday": [
    {
      "time": "10:00-11:30",
      "class": "Algorithms",
      "room": "A-401"
    }
  ]
}
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
