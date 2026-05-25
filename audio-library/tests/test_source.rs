#[cfg(test)]
mod tests {
    use std::path::PathBuf;

use audio_library::sources::{directory::{Directory, DirectoryOptions}, traits::SourceTrack};

    #[test]
    fn test_directory_source() {
        let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests").join("data").join("messy-library");

        assert_eq!(messy_library_path.exists(), true);

        let directory = Directory {
            path: messy_library_path,
            options: DirectoryOptions::default()
        };

        let tracks_iter = directory.get_tracks();

        // TODO: test library parsing and check for correct track metadata
    }
}