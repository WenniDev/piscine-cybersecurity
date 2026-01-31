mod cipher;
mod totp;

use clap::Parser;
use std::{env, fs};

#[derive(Parser)]
#[command(name = "ft_otp")]
struct Cli {
    #[arg(
        short = 'g',
        value_name = "FILE",
        conflicts_with = "key",
        help = "Generate an encrypted key file from a 64 hexadecimal key"
    )]
    generate: Option<String>,

    #[arg(
        short = 'k',
        value_name = "FILE",
        conflicts_with = "generate",
        help = "Use an encrypted key file to generate a TOTP code"
    )]
    key: Option<String>,

    #[arg(
        short = 'q',
        conflicts_with = "key",
        default_value_t = false,
        help = "Output the TOTP code as a QR code"
    )]
    qrcode: bool,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    if cli.generate.is_none() && cli.key.is_none() {
        anyhow::bail!("Error: You must at least specify either -g or -k");
    }

    if let Some(file_path) = cli.generate {
        let data = fs::read(file_path)?;
        let raw_key = String::from_utf8(data.clone())?;
        let key = raw_key.trim();

        if key.len() < 64 || hex::decode(&key).is_err() {
            anyhow::bail!("key must be 64 hexadecimal characters.");
        }

        let passphrase = env::var("PASSPHRASE")?;
        let encrypted_key = cipher::encrypt_key(&data, passphrase.as_str())
            .map_err(|_| anyhow::anyhow!("Error encrypting key."))?;

        let filename = "ft_otp.key";
        fs::write(filename, &encrypted_key)?;

        if cli.qrcode {
            let hex_key = hex::decode(&key).unwrap();
            let base32_key = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &hex_key);
            let otp_uri = format!("otpauth://totp/ft_otp?secret={}&issuer=ft_otp", base32_key);
            let code = qrcode::QrCode::new(otp_uri.as_bytes())?;
            let image = code.render::<qrcode::render::unicode::Dense1x2>().build();
            println!("{}", image);
        }

        println!("Key was successfully saved in {}", filename);
    }

    if let Some(file_path) = cli.key {
        let data = fs::read(file_path)?;
        let passphrase = env::var("PASSPHRASE")?;
        let decrypted_key = cipher::decrypt_key(&data, passphrase.as_str())
            .map_err(|_| anyhow::anyhow!("Error decrypting key."))?;
        let raw_key = String::from_utf8(decrypted_key)?;
        let key = raw_key.trim();

        if key.len() < 64 || hex::decode(&key).is_err() {
            anyhow::bail!("key must be 64 hexadecimal characters.");
        }

        let otp = totp::totp(&hex::decode(&key)?);
        println!("{:06}", otp);
    }

    Ok(())
}
