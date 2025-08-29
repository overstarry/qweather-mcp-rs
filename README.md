# QWeather MCP Server

A simple Model Context Protocol (MCP) server implementation in Rust for weather information.

## Features

This MCP server provides:

### Tools
- **get_weather**: Get current weather information for a specified city
  - Input: `city` (string) - Name of the city
  - Output: Weather information including temperature, humidity, wind speed, and conditions

### Resources
- **weather://forecast**: Weather forecast data
  - Provides 3-day weather forecast with detailed information
  - Returns JSON data with temperature ranges, conditions, humidity, and wind speed

## Usage

### Building the Server
```bash
cargo build --release
```

### Running the Server
```bash
cargo run --bin qweather-mcp-server
```

The server listens on stdin/stdout and communicates using the JSON-RPC protocol as defined by the Model Context Protocol specification.

### Testing the Server
Use the included test script:
```bash
./test_mcp_server.sh
```

Or test manually with JSON-RPC messages:

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

#### List available resources
```json
{"jsonrpc": "2.0", "id": 4, "method": "resources/list", "params": {}}
```

#### Read weather forecast resource
```json
{"jsonrpc": "2.0", "id": 5, "method": "resources/read", "params": {"uri": "weather://forecast"}}
```

## Implementation Notes

This is a minimal MCP server implementation that demonstrates:
- JSON-RPC 2.0 protocol handling
- MCP standard method implementations (initialize, tools/list, tools/call, resources/list, resources/read)
- Async I/O with tokio
- Proper error handling and responses

The weather data is currently simulated. In a production implementation, this would integrate with the QWeather API to provide real weather data.

## Protocol Compliance

This server implements the Model Context Protocol (MCP) specification version 2024-11-05 and supports:
- Server initialization
- Tool listing and execution
- Resource listing and reading
- Proper JSON-RPC error handling

## Development

The project structure:
- `src/main.rs` - MCP server implementation
- `src/lib.rs` - Library code (currently minimal)
- `test_mcp_server.sh` - Test script for manual testing

Run tests:
```bash
cargo test
```

## Future Enhancements

- Integration with real QWeather API
- Additional weather tools (alerts, air quality, etc.)
- Configuration support
- Logging improvements
- Extended weather data resources