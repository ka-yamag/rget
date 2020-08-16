use reqwest;
use std::time::Duration;
use crate::Cli;

const DEFAULT_THREDING: u32 = 10;
const DEFAULT_MIN_CHUNK_SIZE: u32 = 1000000;
const DEFAULT_TIMEOUT_SEC: u64 = 10;

#[derive(Debug)]
pub struct DownLoader {
    url: String,
    chunk_size: u32,
    threding: u32,
    client: reqwest::Client,
}

impl DownLoader {
    pub fn new(cli: Cli) -> Result<DownLoader, reqwest::Error> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SEC))
            .build()?;

        Ok(DownLoader {
            url: cli.url.into_string(),
            chunk_size: cli.chunk_size,
            threding: cli.threding,
            client: client,
        })
    }

    pub async fn init(&mut self) -> Result<&mut DownLoader, reqwest::Error> {
        let response = self.client.head(&self.url).send().await?;

        if let Err(err) = response.error_for_status() {
            return Err(err)
        }

        // Set concurrency default to 10
        if self.threding == 0 {
            self.threding = DEFAULT_THREDING;
        }

        // Set default chunk size
        if self.chunk_size == 0 {
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use url::Url;

    #[test]
    fn init_test() {
        let url_str = "http://test.com";
        let url = Url::parse(url_str).unwrap();

        let c = Cli {
            url: url,
            chunk_size: 5,
            threding: 5,
            out: PathBuf::new(),
        };

        if let Ok(d) = DownLoader::new(c) {
            assert_eq!(d.url, "http://test.com/");
            assert_eq!(d.chunk_size, 5);
            assert_eq!(d.threding, 5);
        }
    }
}
