# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building
```bash
cargo build --release
```

### Running the Server
```bash
cargo run --bin qweather-mcp-server
```

### Testing
```bash
cargo test
```

## Architecture Overview

This is a Model Context Protocol (MCP) server implementation in Rust that provides weather information tools using the official `rmcp` SDK.

### Key Components

- **Binary Entry Point** (`src/main.rs`): Sets up the MCP server with stdio transport and manages the async runtime
- **Server Implementation** (`src/server.rs`): Contains the main `QWeatherMcpServer` struct implementing the MCP protocol using the official Rust SDK with `#[tool]` decorators
- **Library Exports** (`src/lib.rs`): Simple module exports and basic test utilities

### MCP Server Architecture

The server uses the official MCP Rust SDK (`rmcp`) with these patterns:
- `#[tool_router]` macro on the impl block for automatic tool registration
- `#[tool]` decorators on methods for type-safe tool definitions with automatic schema generation
- `ServerHandler` trait implementation for server capabilities and metadata
- `Parameters<T>` wrapper for type-safe argument handling with serde deserialization

### Current Tools

- `get_weather`: Takes a city name and returns simulated weather data (temperature, humidity, wind, conditions)

### Protocol Communication

The server communicates via stdin/stdout using JSON-RPC as per MCP specification. Test with manual JSON-RPC calls:
- Initialize: `{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}`
- List tools: `{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}`
- Call tool: `{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_weather", "arguments": {"city": "Beijing"}}}`

## Implementation Notes

- Weather data is currently simulated - production would integrate with real QWeather API
- Uses modern async/await patterns with Tokio runtime
- Built for type safety with automatic JSON schema generation from Rust types
- Follows official MCP protocol compliance using the standard SDK