# GEI Project Changes

## Summary

Transformed the GEI (GoMaluum Entries Indexer) project into a **simple, focused gRPC server** for storing and retrieving encrypted user schedules.

## What Was Built

### Core Functionality
- ✅ **Two gRPC Endpoints:**
  - `StoreSchedule` - Save encrypted schedule JSON for a username
  - `GetSchedule` - Retrieve and decrypt schedule for a username
  
- ✅ **Encryption:** AES-256-GCM with random nonces
- ✅ **Storage:** SQLite database with simple schema
- ✅ **Server:** Async gRPC server on port 50051

### Architecture

```
gRPC Client → Server → [Encrypt/Decrypt] → SQLite Database
```

### Key Design Decisions

1. **Simplified API** - Removed:
   - ❌ List users endpoint
   - ❌ Delete schedule endpoint  
   - ❌ Health check endpoint
   - ❌ CLI client tool
   - ❌ Search functionality

2. **Focus** - Only two operations: save and retrieve

3. **Security** - All data encrypted at rest with AES-256-GCM

4. **Simplicity** - SQLite for persistence, no complex dependencies

## Project Structure

```
gei/
├── proto/schedule.proto       # gRPC service definition (2 RPCs)
├── src/
│   ├── lib.rs                # Library module
│   ├── crypto.rs             # AES-256-GCM encryption
│   ├── db.rs                 # SQLite operations
│   ├── service.rs            # gRPC service implementation
│   ├── server.rs             # Server binary
│   └── main.rs               # Help/info
├── Cargo.toml                # Dependencies
├── build.rs                  # Protobuf compilation
├── Dockerfile                # Container image
├── docker-compose.yml        # Docker orchestration
├── Makefile                  # Build shortcuts
├── README.md                 # Full documentation
├── QUICKSTART.md             # Getting started guide
└── PROJECT_SUMMARY.md        # Technical overview
```

## Dependencies

- **tonic** - gRPC framework
- **prost** - Protocol Buffers
- **sqlx** - SQLite async driver
- **aes-gcm** - Encryption
- **tokio** - Async runtime
- **serde/serde_json** - JSON handling
- **anyhow** - Error handling
- **chrono** - Timestamps

## Usage

### Start Server
```bash
cargo run --bin gei-server
```

### Store Schedule
```bash
grpcurl -plaintext -d '{
  "username": "alice",
  "schedule_json": "{\"monday\":\"Math 9AM\"}"
}' localhost:50051 schedule.ScheduleIndexer/StoreSchedule
```

### Retrieve Schedule
```bash
grpcurl -plaintext -d '{
  "username": "alice"
}' localhost:50051 schedule.ScheduleIndexer/GetSchedule
```

## Configuration

### Environment Variables
- `DATABASE_URL` - SQLite path (default: `sqlite://schedules.db`)
- `ENCRYPTION_KEY` - AES-256 key (default: auto-generated)

## Testing

All tests pass:
```bash
cargo test
```

Test coverage:
- Encryption/decryption with random nonces
- Database store/retrieve operations
- Update existing records (upsert)

## Production Ready Features

- ✅ Encryption at rest
- ✅ Async/await for performance
- ✅ Docker support
- ✅ Error handling
- ✅ Unit tests
- ✅ Documentation

## Missing (By Design)

- No authentication/authorization (add at application layer)
- No TLS (add via reverse proxy or modify code)
- No user management (not needed)
- No complex querying (out of scope)

## Next Steps

1. Add TLS support if needed
2. Implement auth layer (JWT, API keys)
3. Add monitoring/metrics
4. Set up CI/CD pipeline
5. Deploy to production

## Development

```bash
make build          # Build debug
make build-release  # Build optimized
make test           # Run tests
make server         # Start server
make clean          # Clean artifacts
```

---

**Result:** A focused, production-ready gRPC service for encrypted schedule storage.
