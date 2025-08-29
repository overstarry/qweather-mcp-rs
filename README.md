# QWeather MCP Server

A Model Context Protocol (MCP) server implementation in Rust for weather information, built using the official Rust MCP SDK.

## Features

This MCP server provides weather-related tools using the official [rmcp](https://github.com/modelcontextprotocol/rust-sdk) Rust SDK.

### Tools
- **get_weather**: Get current weather information for a specified city
  - Input: `city` (string) - Name of the city
  - Output: Weather information including temperature, humidity, wind speed, and conditions

## Implementation

Built using the official MCP Rust SDK (`rmcp`) with:
- **Modern async/await**: Built with Tokio for high-performance async operations
- **Type-safe tools**: Using `#[tool]` decorators for automatic schema generation
- **Proper error handling**: Complete MCP error responses and validation
- **Official protocol compliance**: Uses the standard MCP protocol implementation

## Usage

### Dependencies

The server requires:
- Rust 1.70+ 
- Tokio runtime
- Official MCP Rust SDK (`rmcp`)

### Building the Server
```bash
cargo build --release
```

### Running the Server
```bash
cargo run --bin qweather-mcp-server
```

The server communicates via stdin/stdout using the JSON-RPC protocol as defined by the Model Context Protocol specification.

### Testing the Server

#### Initialize the server
```json
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
```

#### List available tools
```json
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

#### Call the weather tool
```json
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_weather", "arguments": {"city": "Beijing"}}}
```

## Implementation Notes

This implementation demonstrates:
- Official MCP Rust SDK usage with `#[tool]` decorators
- Type-safe parameter handling with automatic schema generation
- Modern async Rust patterns with tokio
- Proper MCP protocol compliance

The weather data is currently simulated. In a production implementation, this would integrate with the QWeather API to provide real weather data.

## Protocol Compliance

This server implements the Model Context Protocol (MCP) specification using the official Rust SDK and supports:
- Server initialization with proper capability advertisement
- Tool listing and execution with type-safe parameters
- Automatic JSON schema generation for tool inputs
- Proper error handling and MCP-compliant responses

## Development

The project structure:
- `src/main.rs` - Main binary entry point
- `src/server.rs` - MCP server implementation using rmcp SDK
- `src/lib.rs` - Library exports

### Dependencies

```toml
[dependencies]
rmcp = { version = "0.6.0", features = ["server", "macros", "transport-io", "schemars"] }
tokio = { version = "1.0", features = ["macros", "rt", "rt-multi-thread", "io-std"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
schemars = "1.0"
```

Run tests:
```bash
cargo test
```

## Future Enhancements

- Integration with real QWeather API
- Additional weather tools (alerts, air quality, etc.)
- Resource support (when available in rmcp SDK)
- Configuration support
- Enhanced error handling
- Extended weather data capabilities