use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Simple MCP (Model Context Protocol) server implementation
/// This demonstrates the basic structure of an MCP server
/// 
/// In a real implementation, this would integrate with QWeather API
/// to provide actual weather data

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<Value>,
}

struct QWeatherMcpServer {
    name: String,
    version: String,
}

impl QWeatherMcpServer {
    fn new() -> Self {
        Self {
            name: "qweather-mcp-server".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.id),
            "tools/list" => self.handle_tools_list(request.id),
            "tools/call" => self.handle_tools_call(request.id, request.params),
            "resources/list" => self.handle_resources_list(request.id),
            "resources/read" => self.handle_resources_read(request.id, request.params),
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(json!({
                    "code": -32601,
                    "message": "Method not found"
                })),
            },
        }
    }

    fn handle_initialize(&self, id: Option<Value>) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {}
                },
                "serverInfo": {
                    "name": self.name,
                    "version": self.version
                }
            })),
            error: None,
        }
    }

    fn handle_tools_list(&self, id: Option<Value>) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": [
                    {
                        "name": "get_weather",
                        "description": "Get current weather information for a city",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "city": {
                                    "type": "string",
                                    "description": "Name of the city to get weather for"
                                }
                            },
                            "required": ["city"]
                        }
                    }
                ]
            })),
            error: None,
        }
    }

    fn handle_tools_call(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        if let Some(params) = params {
            if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                match name {
                    "get_weather" => {
                        let city = params
                            .get("arguments")
                            .and_then(|args| args.get("city"))
                            .and_then(|c| c.as_str())
                            .unwrap_or("Unknown");

                        // Simulate weather data - in a real implementation,
                        // this would call the QWeather API
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: Some(json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": format!(
                                            "Weather in {}: Sunny, 22°C\n\
                                            Humidity: 60%\n\
                                            Wind Speed: 10 km/h\n\
                                            Conditions: Clear skies with good visibility",
                                            city
                                        )
                                    }
                                ]
                            })),
                            error: None,
                        }
                    }
                    _ => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(json!({
                            "code": -32602,
                            "message": "Unknown tool"
                        })),
                    },
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(json!({
                        "code": -32602,
                        "message": "Invalid parameters"
                    })),
                }
            }
        } else {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(json!({
                    "code": -32602,
                    "message": "Missing parameters"
                })),
            }
        }
    }

    fn handle_resources_list(&self, id: Option<Value>) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "resources": [
                    {
                        "uri": "weather://forecast",
                        "name": "Weather Forecast",
                        "description": "7-day weather forecast data",
                        "mimeType": "application/json"
                    }
                ]
            })),
            error: None,
        }
    }

    fn handle_resources_read(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        if let Some(params) = params {
            if let Some(uri) = params.get("uri").and_then(|v| v.as_str()) {
                match uri {
                    "weather://forecast" => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({
                            "contents": [
                                {
                                    "uri": "weather://forecast",
                                    "mimeType": "application/json",
                                    "text": json!({
                                        "forecast": [
                                            {
                                                "date": "2024-01-01",
                                                "temperature_high": "25°C",
                                                "temperature_low": "15°C",
                                                "condition": "Partly Cloudy",
                                                "humidity": "65%",
                                                "wind_speed": "12 km/h"
                                            },
                                            {
                                                "date": "2024-01-02",
                                                "temperature_high": "23°C",
                                                "temperature_low": "13°C",
                                                "condition": "Rainy",
                                                "humidity": "80%",
                                                "wind_speed": "15 km/h"
                                            },
                                            {
                                                "date": "2024-01-03",
                                                "temperature_high": "26°C",
                                                "temperature_low": "16°C",
                                                "condition": "Sunny",
                                                "humidity": "55%",
                                                "wind_speed": "8 km/h"
                                            }
                                        ]
                                    }).to_string()
                                }
                            ]
                        })),
                        error: None,
                    },
                    _ => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(json!({
                            "code": -32602,
                            "message": "Unknown resource"
                        })),
                    },
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(json!({
                        "code": -32602,
                        "message": "Invalid parameters"
                    })),
                }
            }
        } else {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(json!({
                    "code": -32602,
                    "message": "Missing parameters"
                })),
            }
        }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("QWeather MCP Server v{} starting...", self.version);
        eprintln!("Server capabilities:");
        eprintln!("  - Tools: get_weather");
        eprintln!("  - Resources: weather://forecast");
        eprintln!("Listening on stdin/stdout for MCP messages...");

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(request) => {
                    let response = self.handle_request(request).await;
                    let response_json = serde_json::to_string(&response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
                Err(e) => {
                    eprintln!("Error parsing JSON-RPC request: {}", e);
                    let error_response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(json!({
                            "code": -32700,
                            "message": "Parse error"
                        })),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = QWeatherMcpServer::new();
    server.run().await
}