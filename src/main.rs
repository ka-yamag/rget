mod downloader;
use downloader::DownLoader;

use std::path::PathBuf;
use url::Url;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str), short = "o", long = "output")]
    out: PathBuf,

    url: Url,

    #[structopt(short = "c", long = "chunk_size", default_value = "0")]
    chunk_size: u32,

    #[structopt(short = "t", long = "threding", default_value = "10")]
    threding: u32,
}

fn main() {
    let args = Cli::from_args();
    println!("{:#?}", args);

    // let d = DownLoader {
    //     url: args.url,
    //     chunk_size: args.chunk_size,
    // }.init();
}
