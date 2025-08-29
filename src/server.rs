use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{ServerCapabilities, ServerInfo, CallToolResult, Content},
    schemars, tool, tool_router,
};

/// QWeather MCP Server implementation using the official Rust SDK
/// 
/// This server provides weather-related tools using the 
/// Model Context Protocol (MCP) with the QWeather API integration.

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetWeatherArgs {
    /// Name of the city to get weather for
    pub city: String,
}

#[derive(Clone)]
pub struct QWeatherMcpServer {
    tool_router: ToolRouter<QWeatherMcpServer>,
}

impl QWeatherMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl QWeatherMcpServer {
    /// Get current weather information for a city
    #[tool(description = "Get current weather information for a city")]
    fn get_weather(&self, Parameters(args): Parameters<GetWeatherArgs>) -> Result<CallToolResult, McpError> {
        // Simulate weather data - in a real implementation,
        // this would call the QWeather API
        let weather_info = format!(
            "Weather in {}: Sunny, 22°C\n\
            Humidity: 60%\n\
            Wind Speed: 10 km/h\n\
            Conditions: Clear skies with good visibility",
            args.city
        );
        
        Ok(CallToolResult::success(vec![Content::text(weather_info)]))
    }
}

impl ServerHandler for QWeatherMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("QWeather MCP Server - provides weather information and forecasts".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}