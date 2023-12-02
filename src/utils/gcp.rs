//! GCP Cloud Run utilities, configuration managers, and the main event listener.
//!
//! This module primarily focuses on GCP deployment helpers and the listener, which is required
//! for the [Cloud Run container runtime contract](https://cloud.google.com/run/docs/container-contract).

use crate::utils::logging;

use std::env;

use tokio::net::TcpListener;

/// Default localhost IP address used by GCP to listen for requests.
const GCP_LOCALHOST: &str = "0.0.0.0";

// Default port used by GCP to listen for requests.
const GCP_DEFAULT_PORT: &str = "8080";

/// The GCP Cloud Run instance configuration.
pub struct CloudRunConfig {
    /// The address that GCP will filter requests to for the
    /// [ingress container](https://cloud.google.com/run/docs/container-contract#startup).
    address: String,
}

impl Default for CloudRunConfig {
    /// Create a default [`CloudRunConfig`] instance in the format of `0.0.0.0:PORT`, where `PORT`
    /// is the port defined by the default value set in GCP (`8080`). This value will also be
    /// reflected in the Dockerfile and compose file(s).
    fn default() -> Self {
        CloudRunConfig {
            address: format!(
                "{}:{}",
                GCP_LOCALHOST,
                env::var("PORT").unwrap_or(GCP_DEFAULT_PORT.into())
            ),
        }
    }
}

/// The GCP Cloud Run instance listener. Each Cloud Run instance is expected to have exactly one ingress
/// container (the container that receives and responds to all messages) and optionally one or more "sidecars"
/// which can handle additional operations or take offloaded data from the ingress container.
///
/// By the [Cloud Run container runtime contract](https://cloud.google.com/run/docs/container-contract), Cloud Run
/// instances are *required* to listen to `0.0.0.0:8080` by default (can be configured if so desired). For the sake
/// of Discord bot development, this should not be the primary focus of the codebase and should instead be handled on
/// a background thread while the bot takes precedence on the main thread.
pub struct CloudRunListener {
    /// The configuration for the given listener instance.
    config: CloudRunConfig,
}

impl Default for CloudRunListener {
    /// Constructs a [`CloudRunListener`] using a default [`CloudRunConfig`] instance.
    fn default() -> Self {
        Self {
            config: CloudRunConfig::default(),
        }
    }
}

impl CloudRunListener {
    /// Listen to GCP requests on, by default, `0.0.0.0:8080`. If the proper values are set
    /// in the [`CloudRunConfig`], this address may be different.
    pub(crate) async fn listen(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.config.address).await?;
        println!("Listening on {}", &self.config.address);

        while let Ok(_) = listener.accept().await { /* connection handling code */ }
        Ok(())
    }
}

/// Create a GCP logger with a JSON format.
pub(crate) fn setup_gcp_logger() -> anyhow::Result<()> {
    log::set_boxed_logger(Box::new(logging::Logger::custom(logging::LogFormat::Json)))?;
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    Ok(())
}
