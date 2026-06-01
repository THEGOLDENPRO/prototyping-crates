use std::{fmt::Display, path::PathBuf};

pub enum SourceType {
    Directory { path: PathBuf },
    Jellyfin { url: String }
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::Directory { path } => write!(
                f,
                "Directory (@ {})",
                path.display()
            ),
            SourceType::Jellyfin { url } => write!(f, "Jellyfin (@ {url}) "),
        }
    }
}