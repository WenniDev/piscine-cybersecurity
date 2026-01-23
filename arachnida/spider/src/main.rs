use std::{collections::HashSet, fs::File, io::Write};

use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, warn};
use regex::Regex;
use url::Url;

const HREF_REGEX: &str = r#"href="([^"]+)""#;
const IMG_REGEX: &str = r#"src="(([^"]+)(?:\.(?:png|jpeg|jpg|gif|bmp)))""#;

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

struct Scrapper {
    output: String,
    regex_img: Regex,
    regex_href: Regex,
    already_scrapped: HashSet<String>,
}

impl Scrapper {
    fn new(output: String) -> Self {
        Scrapper {
            output,
            regex_img: Regex::new(IMG_REGEX).unwrap(),
            regex_href: Regex::new(HREF_REGEX).unwrap(),
            already_scrapped: HashSet::new(),
        }
    }

    fn download_image(&mut self, url: Url) -> Result<()> {
        if self.already_scrapped.contains(url.as_str()) {
            return Ok(());
        } else {
            warn!("Image already scrapped: {}", url);
            self.already_scrapped.insert(url.as_str().to_string());
        }

        let url_str = url.as_str();
        let response = reqwest::blocking::get(url_str)?;
        if response.status().is_success() {
            let content = response.bytes()?;

            let filename = url_str.split('/').last().unwrap();
            let path = format!("{}{}", self.output, filename);
            let mut file = File::create(&path)?;
            file.write_all(&content)?;
        }
        Ok(())
    }

    fn get_images(&mut self, url: &Url, depth: u32, recursive: bool) -> Result<()> {
        info!("Start scraping {}", url);
        if recursive && depth > 0 {
            self.start(&url, depth)
        } else {
            self.start(&url, 1)
        }
    }

    fn get_content(&self, url: &str) -> Result<String> {
        let response = reqwest::blocking::get(url)?;
        let content = response.text()?;
        Ok(content)
    }

    fn start(&mut self, url: &Url, depth: u32) -> Result<()> {
        if depth == 0 {
            return Ok(());
        }

        if self.already_scrapped.contains(url.as_str()) {
            warn!("Already scrapped {}", url);
            return Ok(());
        } else {
            self.already_scrapped.insert(url.as_str().to_string());
        }

        let content = self.get_content(url.as_str())?;

        for cap in self.regex_img.clone().captures_iter(&content) {
            let image_url = cap.get(1).unwrap().as_str();
            info!("Downloading image ({}) `{}`", depth, image_url);
            if let Ok(image_url) = url.join(image_url) {
                self.download_image(image_url)?;
            } else {
                error!("Invalid image URL ({}) {}", depth, image_url);
            }
        }

        for cap in self.regex_href.clone().captures_iter(&content) {
            let next_url = cap.get(1).unwrap().as_str();
            info!("Scraping ({}) {}", depth, next_url);
            if let Ok(new_url) = url.join(next_url) {
                if new_url.domain() == url.domain() {
                    self.start(&new_url, depth - 1)?;
                } else {
                    warn!("Skipping external link ({}) {}", depth, next_url);
                }
            } else {
                error!("Invalid URL ({}) {}", depth, next_url);
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let opt = Args::parse();
    match std::fs::create_dir(&opt.path) {
        Ok(_) => debug!("Directory created"),
        Err(e) => error!("Error creating directory: {}", e),
    }

    let mut scrapper = Scrapper::new(opt.path);
    scrapper.get_images(&opt.url, opt.max_depth, opt.recursive)
}
