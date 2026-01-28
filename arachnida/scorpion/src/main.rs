mod exif;
mod file_info;
mod image;
mod tags;

use clap::Parser;

use crate::exif::{ExifParser, Value};
use crate::file_info::FileInfo;
use crate::tags::Tags;

fn format_value_for_tag(tag: &Tags, value: &Value) -> String {
    match tag {
        Tags::ResolutionUnit => {
            if let Value::Short(v) = value {
                if v.len() == 1 {
                    return match v[0] {
                        1 => "None".to_string(),
                        2 => "inches".to_string(),
                        3 => "cm".to_string(),
                        _ => format!("{}", value),
                    };
                }
            }
            format!("{}", value)
        }
        Tags::GPSLatitude | Tags::GPSLongitude => {
            if let Value::Rational(v) = value {
                if v.len() == 3 {
                    let degrees = v[0].0 as f64 / v[0].1 as f64;
                    let minutes = v[1].0 as f64 / v[1].1 as f64;
                    let seconds = v[2].0 as f64 / v[2].1 as f64;
                    return format!(
                        "{} deg {}' {:.2}\"",
                        degrees as u32, minutes as u32, seconds
                    );
                }
            }
            format!("{}", value)
        }
        _ => format!("{}", value),
    }
}

#[derive(Parser)]
#[command(name = "scorpion")]
#[command(about = "Extract EXIF metadata from image files")]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for (i, file) in args.files.iter().enumerate() {
        if i > 0 {
            println!("========================================");
        }
        let file_info = match FileInfo::from_path(file) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Error reading file info for {}: {}", file, e);
                continue;
            }
        };

        println!("{:<32}: {}", "File Name", file_info.file_name);
        println!("{:<32}: {}", "Directory", file_info.directory);
        println!("{:<32}: {}", "File Size", file_info.format_size());

        if let Some(modified) = file_info.modified {
            println!(
                "{:<32}: {}",
                "File Modification Date/Time",
                modified.format("%Y:%m:%d %H:%M:%S%:z")
            );
        }
        if let Some(accessed) = file_info.accessed {
            println!(
                "{:<32}: {}",
                "File Access Date/Time",
                accessed.format("%Y:%m:%d %H:%M:%S%:z")
            );
        }
        if let Some(created) = file_info.created {
            println!(
                "{:<32}: {}",
                "File Creation Date/Time",
                created.format("%Y:%m:%d %H:%M:%S%:z")
            );
        }

        println!("{:<32}: {}", "File Permissions", file_info.permissions);
        println!("{:<32}: {}", "File Type", file_info.file_type);
        println!(
            "{:<32}: {}",
            "File Type Extension", file_info.file_extension
        );
        println!("{:<32}: {}", "MIME Type", file_info.mime_type);

        let data = match std::fs::read(file) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error reading {}: {}", file, e);
                continue;
            }
        };

        let parser = match ExifParser::new(&data) {
            Ok(p) => p,
            Err(_) => {
                continue;
            }
        };

        println!(
            "{:<32}: {}",
            "Exif Byte Order",
            if parser.is_little_endian() {
                "Little-endian (Intel, II)"
            } else {
                "Big-endian (Motorola, MM)"
            }
        );

        let ifds = match parser.parse() {
            Ok(ifds) => ifds,
            Err(e) => {
                eprintln!("Error parsing EXIF data: {}", e);
                continue;
            }
        };

        for entries in ifds.iter() {
            for entry in entries {
                if entry.tag.is_sub_ifd_pointer() {
                    continue;
                }

                if let Some(value) = &entry.value {
                    let formatted_value = format_value_for_tag(&entry.tag, value);
                    println!("{:<32}: {}", format!("{}", entry.tag), formatted_value);
                }
            }
        }
    }
}
