mod log;
mod scrapper;

use crate::scrapper::Scrapper;

use anyhow::Result;
use clap::Parser;
use log::{debug, error};
use url::Url;

fn parse_url(url: &str) -> Result<Url> {
    Url::parse(url).map_err(|_| anyhow::anyhow!("unable to resolve host address"))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    #[arg(short = 'l', long = "max-depth", default_value = "5")]
    max_depth: u32,

    #[arg(short = 'p', long = "path", default_value = "./data/")]
    path: String,

    #[arg(name = "URL", value_parser = parse_url)]
    url: Url,
}

fn main() -> Result<()> {
    log::init_logger();

    let opt = Args::parse();
    match std::fs::create_dir(&opt.path) {
        Ok(_) => debug!("Directory created"),
        Err(e) => error!("Error creating directory: {}", e),
    }

    let mut scrapper = Scrapper::new(opt.path);
    scrapper.get_images(&opt.url, opt.max_depth, opt.recursive)
}
