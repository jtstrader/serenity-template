use std::env;

use tokio::net::TcpListener;

const GCP_LOCALHOST: &str = "0.0.0.0";
const GCP_DEFAULT_PORT: &str = "8080";

pub(crate) struct CloudRunConfig {
    address: String,
}

impl Default for CloudRunConfig {
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

pub(crate) struct CloudRunListener {
    config: CloudRunConfig,
}

impl Default for CloudRunListener {
    fn default() -> Self {
        Self {
            config: CloudRunConfig::default(),
        }
    }
}

impl CloudRunListener {
    pub(crate) async fn listen(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.config.address).await?;
        println!("Listening on {}", &self.config.address);

        while let Ok(_) = listener.accept().await { /* connection handling code */ }
        Ok(())
    }
}
