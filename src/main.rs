mod image;

use clap::Parser;
use image::Image;

#[derive(Parser)]
#[command(name = "scorpion")]
#[command(about = "Extract EXIF metadata from image files")]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in &args.files {
        println!("=== {} ===", file);

        let data = match std::fs::read(file) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error reading {}: {}", file, e);
                continue;
            }
        };

        let img = match Image::try_from(data.as_slice()) {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Unknown or unsupported image format");
                continue;
            }
        };

        if let Some(exif_data) = img.find_exif(data.as_slice()) {
            println!("Found EXIF data of length: {}", exif_data.len());
        } else {
            println!("No EXIF data found.");
        }
    }
}
