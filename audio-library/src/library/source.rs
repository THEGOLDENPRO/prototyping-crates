use std::path::PathBuf;

// NOTE: I think this might actually get removed in turn of
// just passing a source struct that implements 'Source' trait.

pub enum LibraryLocalSource {
    Directory(PathBuf),
}

pub enum LibraryRemoteSource {
    // Jellyfin(JellyfinServer),
}

/// The source of the audio library.
pub enum LibrarySource {
    /// Audio library from your file system.
    Local(LibraryLocalSource),
    /// Audio library from a remote source or provider (e.g: Jellyfin, Navidrome).
    Remote(LibraryRemoteSource),
}