use sc_api::ScClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), sc_api::error::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sc_api=debug")),
        )
        .init();

    let mut client = ScClient::new(None)?;
    let client_id = client.fetch_client_id().await?;
    tracing::info!(%client_id, "Initialized client");

    let results = client.search("lofi", 2).await?;
    //tracing::info!(count = results.collection.len(), "Search completed");

    Ok(())
}
