use reqwest;
use std::time::Duration;

const DEFAULT_THREDING: u32 = 10;
const DEFAULT_MIN_CHUNK_SIZE: u32 = 1000000;
const DEFAULT_TIMEOUT_SEC: u64 = 10;

#[derive(Debug)]
pub struct DownLoader {
    url: String,
    dest: String,
    chunk_size: u32,
    threding: u32,
    client: reqwest::Client,
}

impl DownLoader {
    pub async fn init(&mut self) -> Result<&mut DownLoader, reqwest::Error> {
        self.client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SEC))
            .build()?;

        let response = self.client.head(&self.url).send().await?;

        if let Err(err) = response.error_for_status() {
            return Err(err)
        }

        // Set concurrency default to 10
        if self.threding == 0 {
            self.threding = 10;
        }

        // Set default chunk size
        if self.chunk_size == 0 {
        }

        Ok(self)
    }
}
