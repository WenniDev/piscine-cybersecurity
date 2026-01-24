use anyhow::Result;
use log::{error, info, warn};
use regex::Regex;
use std::{collections::HashSet, fs::File, io::Write, sync::LazyLock};
use url::Url;

static HREF_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"href="([^"]+)""#).unwrap());
static IMG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:src|href|content)="(([^"]+)(?:\.(?:png|jpeg|jpg|gif|bmp)))""#).unwrap()
});

pub struct Scrapper {
    output: String,
    already_scrapped: HashSet<String>,
}

impl Scrapper {
    pub fn new(output: String) -> Self {
        Scrapper {
            output,
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

    pub fn get_images(&mut self, url: &Url, depth: u32, recursive: bool) -> Result<()> {
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

        for cap in IMG_REGEX.captures_iter(&content) {
            let image_url = cap.get(1).unwrap().as_str();
            info!("Downloading image ({}) `{}`", depth, image_url);
            if let Ok(image_url) = url.join(image_url) {
                self.download_image(image_url)?;
            } else {
                error!("Invalid image URL ({}) {}", depth, image_url);
            }
        }

        for cap in HREF_REGEX.captures_iter(&content) {
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
