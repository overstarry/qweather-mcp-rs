use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

use crate::server::Counter;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = Counter::new()
        .serve(stdio())
        .await
        .inspect_err(|e| println!("Error staring server: {}", e))?;

    // Wait for the service to complete
    service.waiting().await?;
    Ok(())
}
