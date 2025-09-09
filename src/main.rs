use anyhow::Result;
use rmcp::ServiceExt;

use crate::server::Counter;

mod server;

#[cfg(feature = "http")]
use std::sync::Arc;

#[cfg(feature = "http")]
use axum::{routing::any_service, Router};

#[cfg(feature = "http")]
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager,
    StreamableHttpServerConfig,
    StreamableHttpService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Select transport mode via CLI arg "--http" or env var "QWEATHER_MCP_HTTP=1"
    let use_http = std::env::args().any(|a| a == "--http")
        || std::env::var("QWEATHER_MCP_HTTP").map(|v| v == "1").unwrap_or(false);

    if use_http {
        #[cfg(feature = "http")]
        {
            run_http_server().await?;
            return Ok(());
        }
        #[cfg(not(feature = "http"))]
        {
            eprintln!(
                "HTTP mode requested but 'http' feature is disabled (enable with `--features http`)"
            );
        }
    }

    // Default stdio transport
    let service = Counter::new()
        .serve(rmcp::transport::stdio())
        .await
        .inspect_err(|e| println!("Error starting server: {}", e))?;

    service.waiting().await?;
    Ok(())
}

#[cfg(feature = "http")]
async fn run_http_server() -> Result<()> {
    let bind_addr = std::env::var("QWEATHER_MCP_HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    // Session manager and config for the Streamable HTTP server
    let session_manager = Arc::new(LocalSessionManager::default());
    let config = StreamableHttpServerConfig::default();

    // Service factory returns a fresh Counter service per session/request lifecycle
    let stream_service: StreamableHttpService<Counter> = StreamableHttpService::new(
        || Ok(Counter::new()),
        session_manager,
        config,
    );

    // Route all methods/paths to the streamable HTTP service
    let app = Router::new().fallback_service(any_service(stream_service));

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    println!("qweather-mcp-server (HTTP) listening on {}", bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
