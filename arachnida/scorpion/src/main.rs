mod exif;
mod image;

use clap::Parser;
use image::parse_image_type;

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

        let img = match parse_image_type(&data) {
            Ok((_, i)) => i,
            Err(_) => {
                eprintln!("Unknown or unsupported image format");
                continue;
            }
        };

        let exif_data = match img.find_tiff_header(data.as_slice()) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error extracting EXIF data: {}", e);
                continue;
            }
        };

        let tiff_header = match exif::parse_tiff_header(exif_data) {
            Ok((_, offset)) => offset,
            Err(e) => {
                eprintln!("Error parsing tiff header: {}", e);
                continue;
            }
        };
        println!("Byte order: {:?}", tiff_header);
    }
}
