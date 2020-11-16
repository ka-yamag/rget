mod downloader;
use downloader::DownLoader;

use std::path::PathBuf;
use url::Url;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    url: Url,

    #[structopt(parse(from_os_str), short = "o", long = "output")]
    out: PathBuf,

    #[structopt(short, long, default_value = "1000000")]
    chunk_size: u32,

    #[structopt(short, long, default_value = "10")]
    threding: u32,
}

fn main() {
    let mut args = Cli::from_args();
    println!("{:#?}", args);

    let downloader = DownLoader::new(&mut args).expect("invalid params");

    println!("{:#?}", downloader);
}
