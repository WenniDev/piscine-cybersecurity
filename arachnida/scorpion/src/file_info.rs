use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

pub struct FileInfo {
    pub file_name: String,
    pub directory: String,
    pub file_size: u64,
    pub modified: Option<DateTime<Local>>,
    pub accessed: Option<DateTime<Local>>,
    pub created: Option<DateTime<Local>>,
    pub permissions: String,
    pub file_type: String,
    pub file_extension: String,
    pub mime_type: String,
}

impl FileInfo {
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let path_obj = Path::new(path);
        let metadata = fs::metadata(path)?;

        let file_name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let directory = path_obj
            .parent()
            .and_then(|p| p.to_str())
            .map(|s| if s.is_empty() { "." } else { s })
            .unwrap_or(".")
            .to_string();

        let file_size = metadata.len();
        let modified = metadata.modified().ok().map(|t| DateTime::<Local>::from(t));
        let accessed = metadata.accessed().ok().map(|t| DateTime::<Local>::from(t));
        let created = metadata.created().ok().map(|t| DateTime::<Local>::from(t));
        let permissions = Self::format_permissions(&metadata);
        let extension = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let (file_type, mime_type) = match extension.as_str() {
            "jpg" | "jpeg" => ("JPEG", "image/jpeg"),
            "png" => ("PNG", "image/png"),
            "gif" => ("GIF", "image/gif"),
            "bmp" => ("BMP", "image/bmp"),
            _ => ("Unknown", "application/octet-stream"),
        };

        Ok(FileInfo {
            file_name,
            directory,
            file_size,
            modified,
            accessed,
            created,
            permissions,
            file_type: file_type.to_string(),
            file_extension: extension,
            mime_type: mime_type.to_string(),
        })
    }

    #[cfg(unix)]
    fn format_permissions(metadata: &fs::Metadata) -> String {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let user = if mode & 0o400 != 0 { "r" } else { "-" };
        let user_w = if mode & 0o200 != 0 { "w" } else { "-" };
        let user_x = if mode & 0o100 != 0 { "x" } else { "-" };
        let group = if mode & 0o040 != 0 { "r" } else { "-" };
        let group_w = if mode & 0o020 != 0 { "w" } else { "-" };
        let group_x = if mode & 0o010 != 0 { "x" } else { "-" };
        let other = if mode & 0o004 != 0 { "r" } else { "-" };
        let other_w = if mode & 0o002 != 0 { "w" } else { "-" };
        let other_x = if mode & 0o001 != 0 { "x" } else { "-" };
        format!(
            "-{}{}{}{}{}{}{}{}{}",
            user, user_w, user_x, group, group_w, group_x, other, other_w, other_x
        )
    }

    #[cfg(not(unix))]
    fn format_permissions(metadata: &fs::Metadata) -> String {
        if metadata.permissions().readonly() {
            "-r--r--r--".to_string()
        } else {
            "-rw-rw-rw-".to_string()
        }
    }

    pub fn format_size(&self) -> String {
        if self.file_size < 1024 {
            format!("{} bytes", self.file_size)
        } else if self.file_size < 1024 * 1024 {
            format!("{:.1} kB", self.file_size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.file_size as f64 / (1024.0 * 1024.0))
        }
    }
}
