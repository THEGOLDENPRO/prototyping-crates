use std::path::PathBuf;

pub enum LocalSource {
    Directory(PathBuf),
}

pub enum RemoteSource {
    // Jellyfin(JellyfinServer),
}

/// The source of the audio library.
pub enum Source {
    /// Audio library from your file system.
    Local(LocalSource),
    /// Audio library from a remote source or provider (e.g: Jellyfin, Navidrome).
    Remote(RemoteSource),
}