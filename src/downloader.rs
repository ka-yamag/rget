use reqwest;
use std::time::Duration;
use crate::Cli;
use anyhow::anyhow;

const DEFAULT_THREDING: u32 = 10;
const DEFAULT_MIN_CHUNK_SIZE: u32 = 1000000;
const DEFAULT_TIMEOUT_SEC: u64 = 10;

#[derive(Debug)]
pub struct DownLoader<'a> {
    url: &'a str,
    chunk_size: u32,
    threding: u32,
}

impl<'a> DownLoader<'a> {
    pub fn new(cli: &mut Cli) -> anyhow::Result<DownLoader> {
        let mut rt = tokio::runtime::Runtime::new()?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SEC))
            // .proxy()
            .build()?;

        let s = async {
            client.head(cli.url.path())
                .send()
                .await
        };

        let response = rt.block_on(s)?;

        if let Err(err) = response.error_for_status() {
            return Err(anyhow!(err))
        }

        // Set concurrency default to 10
        if cli.threding <= 0 {
            cli.threding = DEFAULT_THREDING;
        }

        // Set default chunk size
        if cli.chunk_size <= 0 {
            cli.chunk_size = DEFAULT_MIN_CHUNK_SIZE;
        }

        Ok(DownLoader {
            url: cli.url.as_str(),
            chunk_size: cli.chunk_size,
            threding: cli.threding,
        })
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

        let mut c = Cli {
            url: url,
            chunk_size: 5,
            threding: 5,
            out: PathBuf::new(),
        };

        if let Ok(d) = DownLoader::new(&mut c) {
            assert_eq!(d.url, "http://test.com/");
            assert_eq!(d.chunk_size, 5);
            assert_eq!(d.threding, 5);
        }
    }
}
