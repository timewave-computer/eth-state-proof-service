# Ethereum State Proof Service

A Rust-based microservice that provides Ethereum state proofs for account and storage data. This service allows you to generate Merkle proofs for Ethereum state data at specific block heights, which can be used for zero-knowledge verification of Ethereum state.

## Features

- Generate account proofs for Ethereum addresses
- Generate storage proofs for specific storage slots
- RESTful API interface
- CORS support for cross-origin requests
- JSON request/response format

## Prerequisites

- Rust 2024 edition or later
- Access to an Ethereum node RPC endpoint (e.g., Infura, Alchemy)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/eth-state-proof-service.git
cd eth-state-proof-service
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Running the Service

Start the service with:
```bash
cargo run --release
```

The service will start on port 7777 by default.

### API Endpoints

#### POST /

Generate a state proof for an Ethereum address or storage slot.

**Request Body:**
```json
{
    "address": "0x1234...",  // Ethereum address (0x-prefixed hex)
    "ethereum_url": "https://eth-mainnet.alchemyapi.io/v2/your-api-key",  // RPC URL
    "height": 12345678,  // Block height
    "key": "0x..."  // Optional: Storage slot key (0x-prefixed hex)
}
```

**Response:**
```json
{
    "domain": "ethereum",
    "root": "0x...",  // Merkle root (currently zero)
    "payload": [],
    "proof": "..."  // Serialized proof bytes
}
```

### Error Handling

The service returns appropriate HTTP status codes:
- 200: Success
- 400: Invalid request format
- 500: Internal server error

## Dependencies

- `common-merkle-proofs`: Common Merkle proof utilities
- `ethereum-merkle-proofs`: Ethereum-specific Merkle proof implementation
- `valence-coprocessor`: State proof handling
- `axum`: Web framework
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `tower-http`: HTTP middleware (CORS)
- `reqwest`: HTTP client

## Development

### Project Structure

```
.
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock         # Dependency lock file
├── src/
│   ├── main.rs        # Main application entry point
│   └── util.rs        # Utility functions for state proof generation
└── examples/          # Example usage
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```