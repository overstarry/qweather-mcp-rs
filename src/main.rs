use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

mod server;
use server::QWeatherMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an instance of our QWeather MCP server
    let service = QWeatherMcpServer::new().serve(stdio()).await?;

    // Wait for the service to complete
    service.waiting().await?;
    Ok(())
}