use crate::server::shiitake_uranai_mcp_server::ShiitakeUranai;
use crate::shiitake_domain::constellation::Constellation;
use anyhow::Result;
use rmcp::{ServiceExt, transport};
use std::{env, str::FromStr};
use tracing_subscriber::{self, EnvFilter};
mod server;
mod shiitake_domain;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting shiitake_uranai_mcp_server...");

    let constellation_str = env::var("CONSTELLATION").map_err(|_| {
        tracing::error!("CONSTELLATION environment variable not set");
        anyhow::anyhow!("CONSTELLATION environment variable is required")
    })?;

    let constellation = Constellation::from_str(&constellation_str).map_err(|_| {
        tracing::error!("Invalid constellation: {}", constellation_str);
        anyhow::anyhow!("CONSTELLATION must be one of: aries, taurus, gemini, cancer, leo, virgo, libra, scorpio, sagittarius, capricorn, aquarius, pisces")
    })?;

    let service = ShiitakeUranai::new(constellation)
        .serve(transport::stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
