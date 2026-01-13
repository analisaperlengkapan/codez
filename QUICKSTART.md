# Codeza - Quick Start Guide

## Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- Cargo (comes with Rust)
- 2GB free disk space
- Linux/macOS/Windows (with WSL for Windows)

## Installation & Setup

### 1. Clone the Repository
```bash
git clone https://github.com/analisaperlengkapan/codeza.git
cd codeza
```

### 2. Install Rust (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Build the Project
```bash
cargo build
```

This will compile:
- Backend (Axum API server)
- Frontend (Leptos SSR application)
- Shared types

Build time: ~2-3 minutes on first build, ~10-20s on subsequent builds.

## Running the Application

### Start Backend Server
```bash
cargo run --bin backend
```

Output:
```
listening on 127.0.0.1:3000
```

The server is now running and ready to accept requests.

### Test the API
```bash
# In a new terminal
bash test_api.sh
```

You should see:
```
Passed: 56
Failed: 0
Total: 56
```

## API Examples

### Get All Repositories
```bash
curl http://127.0.0.1:3000/api/v1/repos | jq
```

### Create a New Repository
```bash
curl -X POST http://127.0.0.1:3000/api/v1/user/repos \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-awesome-repo",
    "description": "My awesome project",
    "private": false,
    "auto_init": true
  }' | jq
```

### List Issues
```bash
curl http://127.0.0.1:3000/api/v1/repos/admin/codeza/issues | jq
```

### Create an Issue
```bash
curl -X POST http://127.0.0.1:3000/api/v1/repos/admin/codeza/issues \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Add feature X",
    "body": "This feature would improve..."
  }' | jq
```

### Login User
```bash
curl -X POST http://127.0.0.1:3000/api/v1/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password"
  }' | jq
```

## Project Structure

```
codeza/
├── crates/
│   ├── backend/          # Axum REST API
│   │   ├── src/
│   │   │   ├── main.rs          # Entry point
│   │   │   ├── router.rs        # Route definitions & AppState
│   │   │   ├── error.rs         # Error handling (NEW)
│   │   │   ├── validation.rs    # Input validation (NEW)
│   │   │   ├── handlers/        # Request handlers
│   │   │   │   ├── repo.rs
│   │   │   │   ├── user.rs
│   │   │   │   ├── admin.rs
│   │   │   │   ├── project.rs
│   │   │   │   ├── package.rs
│   │   │   │   └── action.rs
│   │   │   └── tests.rs
│   │   └── Cargo.toml
│   ├── frontend/         # Leptos SSR app
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── components/
│   │   │   └── pages/
│   │   └── Cargo.toml
│   └── shared/           # Shared types
│       ├── src/lib.rs
│       └── Cargo.toml
├── test_api.sh           # API test suite (NEW)
├── COMPLETION_REPORT.md  # Project report (NEW)
├── IMPROVEMENTS.md       # Improvements doc (NEW)
├── ARCHITECTURE.md       # Architecture guide (NEW)
├── STANDARDS.md          # Coding standards (NEW)
├── Cargo.toml            # Workspace config
├── Cargo.lock
└── README.md
```

## Testing

### Run API Tests
```bash
bash test_api.sh
```

### Run Rust Unit Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Check Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy
```

## Common Commands

### Development
```bash
# Run backend in development mode
cargo run --bin backend

# Run with logging
RUST_LOG=debug cargo run --bin backend

# Watch mode (auto-rebuild)
cargo watch -x "run --bin backend"
```

### Building
```bash
# Debug build (faster, larger binary)
cargo build

# Release build (slower, optimized, smaller)
cargo build --release

# Build backend only
cargo build --bin backend

# Build frontend only
cargo build --bin frontend
```

### Testing & Quality
```bash
# Run all tests
cargo test --all

# Test specific crate
cargo test -p backend

# Run with specific test name
cargo test test_create_repo

# Check formatting
cargo fmt --all -- --check

# Fix formatting
cargo fmt --all

# Lint code
cargo clippy --all

# Run linting with strict warnings
cargo clippy --all -- -D warnings
```

## Configuration

### Backend Settings
Edit `crates/backend/src/main.rs` to change:
- **Port**: Line 15 (`([127, 0, 0, 1], 3000)`)
- **Bind address**: `127.0.0.1` to `0.0.0.0` for external access

### Feature Flags
```toml
# crates/backend/Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
# Available features: rt, sync, io-util, fs, net, time, macros, etc.
```

## Troubleshooting

### Error: "Address already in use"
The backend is already running on port 3000.
```bash
# Kill existing process
pkill -f "target/debug/backend"

# Or use different port (requires code change)
```

### Error: "Cannot find module"
Clean and rebuild:
```bash
cargo clean
cargo build
```

### Compilation takes too long
Use parallel compilation:
```bash
CARGO_BUILD_JOBS=8 cargo build
```

### Tests failing
Ensure backend is running:
```bash
# Terminal 1
cargo run --bin backend

# Terminal 2
bash test_api.sh
```

## Performance Tips

### Faster Builds
```bash
# Install sccache (compile cache)
cargo install sccache
export RUSTC_WRAPPER=sccache

# Use mold linker (Linux)
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build
```

### Faster Testing
```bash
# Run tests in parallel
cargo test --all -- --test-threads=8

# Skip doc tests
cargo test --lib
```

## Next Steps

1. **Read the Documentation**
   - [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture
   - [STANDARDS.md](./STANDARDS.md) - Coding standards
   - [IMPROVEMENTS.md](./IMPROVEMENTS.md) - Improvement roadmap

2. **Try the API**
   - Use `bash test_api.sh` to see all endpoints
   - Try the curl examples above
   - Read handler code to understand implementation

3. **Modify the Code**
   - Add a new API endpoint in `handlers/`
   - Create a new route in `router.rs`
   - Test with `bash test_api.sh`

4. **Prepare for Production** (see COMPLETION_REPORT.md)
   - Add database layer
   - Implement authentication
   - Add structured logging
   - Docker containerization

## Key Files to Know

| File | Purpose |
|------|---------|
| `src/main.rs` | Application entry point |
| `src/router.rs` | Route definitions and AppState |
| `src/error.rs` | Error handling framework |
| `src/validation.rs` | Input validation |
| `src/handlers/*.rs` | Request handlers by feature |
| `test_api.sh` | Comprehensive API tests |

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [Leptos Documentation](https://leptos.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Support

For issues or questions:
1. Check `COMPLETION_REPORT.md` for known limitations
2. Review `ARCHITECTURE.md` for design decisions
3. Check `STANDARDS.md` for coding guidelines
4. Run tests to verify functionality: `bash test_api.sh`

---

**Happy Coding! 🚀**
