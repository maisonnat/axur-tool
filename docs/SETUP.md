# Development Setup

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install wasm-pack trunk cargo-audit
```

## Running the Project

### Backend
```bash
cargo run -p axur-backend
# Server runs at http://localhost:3001
```

### Frontend
```bash
cd crates/frontend
trunk serve
# Frontend runs at http://localhost:8080
```
