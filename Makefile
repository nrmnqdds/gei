.PHONY: help build build-release test clean server reset-db install format check

# Default target
help:
	@echo "🔧 GEI - GoMaluum Entries Indexer"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          - Build the project in debug mode"
	@echo "  make build-release  - Build optimized release version"
	@echo "  make test           - Run all tests"
	@echo "  make check          - Run cargo check"
	@echo "  make format         - Format code with rustfmt"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make server         - Run the gRPC server"
	@echo "  make reset-db       - Delete database file"
	@echo "  make install        - Install binary to ~/.cargo/bin"
	@echo ""

# Build in debug mode
build:
	@echo "🔨 Building project..."
	@cargo build
	@echo "✅ Build complete"

# Build in release mode
build-release:
	@echo "🔨 Building release version..."
	@cargo build --release
	@echo "✅ Release build complete"
	@echo "📦 Binary available at: target/release/gei-server"

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test
	@echo "✅ Tests complete"

# Check code without building
check:
	@echo "🔍 Checking code..."
	@cargo check
	@echo "✅ Check complete"

# Format code
format:
	@echo "🎨 Formatting code..."
	@cargo fmt
	@echo "✅ Format complete"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@echo "✅ Clean complete"

# Run the server
server:
	@echo "🚀 Starting GEI Server..."
	@cargo run --bin gei-server

# Reset database
reset-db:
	@echo "⚠️  Resetting database..."
	@rm -f schedules.db schedules.db-shm schedules.db-wal
	@echo "✅ Database deleted"

# Install binary
install: build-release
	@echo "📦 Installing binary..."
	@cargo install --path .
	@echo "✅ Installation complete"
	@echo "You can now run: gei-server"

run:
	@echo "🚀 Running GEI Server..."
	@RUST_LOG=trace cargo run --bin gei-server
